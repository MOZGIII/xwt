//! The build dependency for the xwt test assets.

#![cfg(not(target_family = "wasm"))]
#![allow(missing_docs, clippy::missing_docs_in_private_items)]

use std::{
    io::ErrorKind,
    matches,
    path::{Path, PathBuf},
};

use p256::pkcs8::{
    der::{Encode, EncodePem},
    EncodePrivateKey,
};

pub type AnyBytes = Box<dyn AsRef<[u8]>>;

pub const GENERATED_AT_FILE_NAME: &str = "generated-at.txt";

pub fn generate() -> Vec<(&'static str, AnyBytes)> {
    let params =
        xwt_cert_gen::Params::new("xwt-test-certificate", &["localhost", "127.0.0.1", "::1"]);

    let key = p256::SecretKey::random(&mut rand::thread_rng());
    let key = p256::ecdsa::SigningKey::from(key);
    let cert = params
        .self_signed::<_, p256::ecdsa::DerSignature>(&key)
        .unwrap();

    let key: p256::SecretKey = key.into();

    let line_ending = p256::pkcs8::LineEnding::LF;

    let generated_at = time::UtcDateTime::now();
    let generated_at_string = generated_at
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap();

    vec![
        ("cert.der", Box::new(cert.to_der().unwrap())),
        ("cert.pem", Box::new(cert.to_pem(line_ending).unwrap())),
        ("key-sec1.der", Box::new(key.to_sec1_der().unwrap())),
        (
            "key-sec1.pem",
            Box::new(key.to_sec1_pem(line_ending).unwrap()),
        ),
        (
            "key-pkcs8.der",
            Box::new(key.to_pkcs8_der().unwrap().to_bytes()),
        ),
        (
            "key-pkcs8.pem",
            Box::new(key.to_pkcs8_pem(line_ending).unwrap()),
        ),
        (GENERATED_AT_FILE_NAME, Box::new(generated_at_string)),
    ]
}

pub fn env_dir(key: &str) -> PathBuf {
    PathBuf::from(std::env::var_os(key).unwrap())
}

#[derive(Debug)]
pub enum SaveOutcome {
    AllFilesWerePresent,
    AllFilesWereWritten,
}

pub fn save(dir: impl AsRef<Path>, files: Vec<(&'static str, AnyBytes)>) -> SaveOutcome {
    use std::io::Write;

    let dir = dir.as_ref();

    std::fs::create_dir_all(dir).unwrap();

    let expiration_check_result = check_for_expired_generation(dir);

    let open = |file| {
        std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(file)
    };

    let paths = files.iter().map(|(file_name, _)| dir.join(file_name));

    for path in paths.clone() {
        println!("cargo::rerun-if-changed={}", path.display());
    }

    let results: Vec<_> = paths.map(open).collect();

    let all_files_exist = results
        .iter()
        .all(|result| matches!(result, Err(err) if err.kind() == ErrorKind::AlreadyExists));

    // *Before* writing the files, check if all the files are present -
    // as maybe we don't need to do anything.
    // But only when the stuff we have is not expired...
    if let ExpirationCheckResult::NotStale = expiration_check_result {
        if all_files_exist {
            // We should check if the files are expired here and force-replace
            // them if needed - but I'll skip this for now.
            return SaveOutcome::AllFilesWerePresent;
        }
    }

    eprintln!("Saving new xwt test assets");

    for (result, (file_name, data)) in results.into_iter().zip(files.into_iter()) {
        let mut file = match result {
            Ok(file) => file,
            Err(err) if err.kind() == ErrorKind::AlreadyExists => std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(dir.join(file_name))
                .unwrap(),
            other => other.unwrap(),
        };

        file.write_all((*data).as_ref()).unwrap();
        file.flush().unwrap();
    }

    SaveOutcome::AllFilesWereWritten
}

enum ExpirationCheckResult {
    ShouldRegenerate,
    NotStale,
}

fn check_for_expired_generation(dir: &Path) -> ExpirationCheckResult {
    let generated_at_string = match std::fs::read_to_string(dir.join(GENERATED_AT_FILE_NAME)) {
        Ok(val) => val,
        Err(err) if err.kind() == ErrorKind::NotFound => {
            return ExpirationCheckResult::ShouldRegenerate
        }
        other => other.unwrap(),
    };

    if generated_at_string.is_empty() {
        eprintln!("The xwt test assets are not present and should be created");
        return ExpirationCheckResult::ShouldRegenerate;
    }

    let generated_at = time::UtcDateTime::parse(
        generated_at_string.trim_ascii(),
        &time::format_description::well_known::Rfc3339,
    )
    .unwrap();

    let now = time::UtcDateTime::now();

    let generated_expires_at =
        generated_at.saturating_add(time::Duration::weeks(2) - time::Duration::days(1));

    if now > generated_expires_at {
        eprintln!("The xwt test assets are expired and should be updated");
        return ExpirationCheckResult::ShouldRegenerate;
    }

    eprintln!("The xwt test assets are up-to-date");
    ExpirationCheckResult::NotStale
}

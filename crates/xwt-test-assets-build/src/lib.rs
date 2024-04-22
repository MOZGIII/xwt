//! The build dependency for the xwt test assets.

#![cfg(not(target_family = "wasm"))]
#![allow(missing_docs)]

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

    let open = |file| {
        std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(file)
    };

    let results: Vec<_> = files
        .iter()
        .map(|(file_name, _)| open(dir.join(file_name)))
        .collect();

    // *Before* writing the files, check if all the files are in valid state -
    // as maybe we don't need to do anything.
    if results
        .iter()
        .all(|result| matches!(result, Err(err) if err.kind() == ErrorKind::AlreadyExists))
    {
        // We should check if the files are expired here and force-replace
        // them if needed - but I'll skip this for now.
        return SaveOutcome::AllFilesWerePresent;
    }

    for (result, (_, data)) in results.into_iter().zip(files.into_iter()) {
        let mut file = result.unwrap();
        file.write_all((*data).as_ref()).unwrap();
        file.flush().unwrap();
    }

    SaveOutcome::AllFilesWereWritten
}

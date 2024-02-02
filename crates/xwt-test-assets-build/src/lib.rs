//! The build dependency for the xwt test assets.

#![cfg(not(target_family = "wasm"))]
#![allow(missing_docs)]

use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

pub fn generate() -> rcgen::Certificate {
    let params = xwt_cert_utils::gen::Params {
        common_name: "xwt test certificate",
        subject_alt_names: &["localhost", "127.0.0.1", "::1"],
        valid_days_before: 0,
        valid_days_after: 14,
    };

    let alg = &rcgen::PKCS_ECDSA_P256_SHA256;
    let key = rcgen::KeyPair::generate(alg).unwrap();
    params.into_rcgen_cert(key)
}

pub fn state_dir() -> PathBuf {
    let mut dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    dir.push("assets");
    println!("{}", dir.display());
    dir
}

#[cfg(feature = "tokio")]
pub async fn save_tokio(certificate: rcgen::Certificate, dir: impl AsRef<Path>) {
    use tokio::io::AsyncWriteExt;

    let dir = dir.as_ref();

    tokio::fs::create_dir_all(dir).await.unwrap();

    let open = |file| async move {
        tokio::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(file)
            .await
    };

    let results = (
        open(dir.join("cert.der")).await,
        open(dir.join("key.der")).await,
    );

    match results {
        (Ok(mut cert_file), Ok(mut key_file)) => {
            cert_file
                .write_all(&certificate.serialize_der().unwrap())
                .await
                .unwrap();
            key_file
                .write_all(&certificate.serialize_private_key_der())
                .await
                .unwrap();
            cert_file.flush().await.unwrap();
            key_file.flush().await.unwrap();
        }
        (Err(cert_err), Err(key_err))
            if cert_err.kind() == ErrorKind::AlreadyExists
                && key_err.kind() == ErrorKind::AlreadyExists => {}
        (cert_res, key_res) => {
            let _ = cert_res.unwrap();
            let _ = key_res.unwrap();
        }
    }
}

pub fn save(certificate: rcgen::Certificate, dir: impl AsRef<Path>) {
    use std::io::Write;

    let dir = dir.as_ref();

    std::fs::create_dir_all(dir).unwrap();

    let open = |file| {
        std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(file)
    };

    let results = (open(dir.join("cert.der")), open(dir.join("key.der")));

    match results {
        (Ok(mut cert_file), Ok(mut key_file)) => {
            cert_file
                .write_all(&certificate.serialize_der().unwrap())
                .unwrap();
            key_file
                .write_all(&certificate.serialize_private_key_der())
                .unwrap();
            cert_file.flush().unwrap();
            key_file.flush().unwrap();
        }
        (Err(cert_err), Err(key_err))
            if cert_err.kind() == ErrorKind::AlreadyExists
                && key_err.kind() == ErrorKind::AlreadyExists => {}
        (cert_res, key_res) => {
            let _ = cert_res.unwrap();
            let _ = key_res.unwrap();
        }
    }
}

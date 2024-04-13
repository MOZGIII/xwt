//! The build dependency for the xwt test assets.

#![cfg(not(target_family = "wasm"))]
#![allow(missing_docs)]

use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

use p256::pkcs8::der::Encode;

/// A pair of a key and a certificate.
type KeyCert = (p256::SecretKey, xwt_cert_gen::x509_cert::Certificate);

pub fn generate() -> KeyCert {
    let params =
        xwt_cert_gen::Params::new("xwt-test-certificate", &["localhost", "127.0.0.1", "::1"]);

    let key = p256::SecretKey::random(&mut rand::thread_rng());
    let key = p256::ecdsa::SigningKey::from(key);
    let cert = params
        .self_signed::<_, p256::ecdsa::DerSignature>(&key)
        .unwrap();

    (key.into(), cert)
}

pub fn env_dir(key: &str) -> PathBuf {
    PathBuf::from(std::env::var_os(key).unwrap())
}

#[cfg(feature = "tokio")]
pub async fn save_tokio((key, cert): KeyCert, dir: impl AsRef<Path>) {
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
            cert_file.write_all(&cert.to_der().unwrap()).await.unwrap();
            key_file
                .write_all(&key.to_sec1_der().unwrap())
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

pub fn save((key, cert): KeyCert, dir: impl AsRef<Path>) {
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
            cert_file.write_all(&cert.to_der().unwrap()).unwrap();
            key_file.write_all(&key.to_sec1_der().unwrap()).unwrap();
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

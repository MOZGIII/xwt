pub use base64::engine::general_purpose::STANDARD as Base64Engine;

/// A type that represents a PEM-encoded public key.
pub struct PrivateKey(pub String);

/// A type that represents a PEM-encoded certificate.
pub struct Certificate(pub String);

/// Parse the certificate in PEM format.
pub fn parse(data: &str) -> Result<Vec<u8>, pem::PemError> {
    let parsed = pem::parse(data)?;
    let data = parsed.into_contents();
    Ok(data)
}

#[cfg(feature = "rcgen")]
pub fn from_rcgen(cert: &rcgen::Certificate) -> (PrivateKey, Certificate) {
    let key_pem = cert.serialize_private_key_pem();
    let cert_pem = cert.serialize_pem().unwrap();
    (PrivateKey(key_pem), Certificate(cert_pem))
}

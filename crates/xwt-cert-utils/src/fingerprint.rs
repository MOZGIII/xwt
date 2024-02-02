//! Fingerprint computation.

/// RFC7469 fingerprint.
#[cfg(feature = "fingerprint-rfc7469")]
pub fn rfc7469(cert: &rcgen::Certificate) -> String {
    use base64::{engine::general_purpose::STANDARD as Base64Engine, Engine};

    let digest = crate::digest::sha256(&cert.get_key_pair().public_key_der());
    Base64Engine.encode(digest)
}

/// SHA256 fingerprint.
///
/// Note for someone who wants to verify the output, it should be similar to
/// the output for the following command:
/// ```shell
/// openssl x509 -noout -fingerprint -sha256 -in certificate.pem
/// ```
#[cfg(feature = "fingerprint-sha256")]
pub fn sha256(cert_der: &[u8]) -> String {
    let digest = crate::digest::sha256(cert_der);
    let digest: &[u8] = digest.as_ref();

    digest
        .iter()
        .map(|byte| format!("{:X}", byte))
        .collect::<Vec<_>>()
        .join(":")
}

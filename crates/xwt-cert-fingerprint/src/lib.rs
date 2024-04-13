//! Fingerprint computation.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

/// RFC 7469 fingerprint.
///
/// Only available if the `alloc` feature is active.
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rfc7469;

#[cfg(feature = "alloc")]
impl Rfc7469 {
    /// Compute RFC 7469 signature for the [`x509_cert::Certificate`].
    #[cfg(feature = "x509-cert")]
    pub fn compute_x509_cert(cert: &x509_cert::Certificate) -> alloc::string::String {
        Self::compute_for_public_key_bytes(
            cert.tbs_certificate
                .subject_public_key_info
                .subject_public_key
                .raw_bytes(),
        )
    }

    /// Compute RFC 7469 signature for the [`rustls_pki_types::TrustAnchor`].
    #[cfg(feature = "rustls-pki-types")]
    pub fn compute_rustls_pki_types(
        cert: &rustls_pki_types::TrustAnchor<'_>,
    ) -> alloc::string::String {
        Self::compute_for_public_key_bytes(&cert.subject_public_key_info)
    }

    /// Compute RFC 7469 signature for the given public key bytes.
    pub fn compute_for_public_key_bytes(public_key_bytes: &[u8]) -> alloc::string::String {
        use base64::Engine;
        use sha2::Digest;

        let digest = sha2::Sha256::digest(public_key_bytes);
        base64::engine::general_purpose::STANDARD.encode(digest)
    }
}

/// Alloc-free writer of a der encoded data into the digest.
#[cfg(feature = "x509-cert")]
struct DigestDerWriter<T>(pub T);

#[cfg(feature = "x509-cert")]
impl<T> x509_cert::der::Writer for DigestDerWriter<T>
where
    T: sha2::Digest,
{
    fn write(&mut self, slice: &[u8]) -> x509_cert::der::Result<()> {
        self.0.update(slice);
        Ok(())
    }
}

/// A SHA-256 fingerprint of the certificate.
///
/// Note for anyone who wants to verify the output, it should be similar to
/// the output for the following command:
/// ```shell
/// openssl x509 -noout -fingerprint -sha256 -in certificate.pem
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sha256([u8; 32]);

/// A reference to a SHA-256 fingerprint of the certificate.
///
/// Note for anyone who wants to verify the output, it should be similar to
/// the output for the following command:
/// ```shell
/// openssl x509 -noout -fingerprint -sha256 -in certificate.pem
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sha256Ref<'a>(&'a [u8; 32]);

impl Sha256 {
    /// Compute the SHA-256 fingerprint of the given [`x509_cert::Certificate`].
    #[cfg(feature = "x509-cert")]
    pub fn compute_x509_cert(cert: &x509_cert::Certificate) -> Self {
        use sha2::Digest;
        use x509_cert::der::Encode;

        let digest = sha2::Sha256::new();
        let mut digest = DigestDerWriter(digest);
        cert.encode(&mut digest).unwrap();
        let DigestDerWriter(digest) = digest;
        Self::from_fingerprint_bytes(digest.finalize().into())
    }

    /// Compute the SHA-256 fingerprint of the given
    /// [`rustls_pki_types::CertificateDer`].
    #[cfg(feature = "rustls-pki-types")]
    pub fn compute_rustls_pki_types(cert: rustls_pki_types::CertificateDer<'_>) -> Self {
        Self::compute_for_der(&cert)
    }

    /// Compute the SHA-256 fingerprint of the certificate data encoded
    /// in the DER format.
    pub fn compute_for_der(cert: &[u8]) -> Self {
        use sha2::Digest;
        Self::from_fingerprint_bytes(sha2::Sha256::digest(cert).into())
    }

    /// Create a [`Sha256`] fingerprint object from the precomputed fingerprint
    /// bytes.
    /// Might be useful to someone who want to use the display implementation.
    pub const fn from_fingerprint_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Return the 32-byte array of the signature data.
    pub const fn into_inner(self) -> [u8; 32] {
        self.0
    }

    /// Return the reference to this signature.
    pub const fn as_ref(&self) -> Sha256Ref<'_> {
        Sha256Ref(&self.0)
    }
}

impl<'a> Sha256Ref<'a> {
    /// Create a [`Sha256Ref`] fingerprint object from the precomputed
    /// fingerprint bytes.
    /// Might be useful to someone who want to use the display implementation.
    pub const fn from_fingerprint_bytes(bytes: &'a [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Return a ref to the 32-byte array of the signature data.
    pub const fn into_inner(self) -> &'a [u8; 32] {
        self.0
    }
}

impl core::fmt::Display for Sha256 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<'a> core::fmt::Display for Sha256Ref<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut first = true;
        for byte in self.0 {
            if first {
                first = false;
            } else {
                f.write_str(":")?;
            }
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

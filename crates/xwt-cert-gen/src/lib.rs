//! Certificate generation facilities.
//!
//! See [`Params::self_signed`] for a usage example.

pub use x509_cert;

/// The params for certificate generation.
#[allow(missing_docs)]
#[derive(Debug)]
pub struct Params<'a> {
    pub common_name: &'a str,
    pub subject_alt_names: &'a [&'a str],
    pub valid_days_before: u32,
    pub valid_days_after: u32,
}

impl<'a> Params<'a> {
    /// Create new certificate generation params with the given common name
    /// and subject alt names and the standard WebTransport self-signed
    /// certificate two-week validity window.
    pub fn new(common_name: &'a str, subject_alt_names: &'a [&'a str]) -> Self {
        Self {
            common_name,
            subject_alt_names,
            valid_days_before: 0,
            valid_days_after: 14,
        }
    }
}

/// A duration of a single day.
const DAY: std::time::Duration = std::time::Duration::from_secs(60 * 60 * 24);

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("bad common name value: {0}")]
    BadCommonName(der::Error),
    #[error("bad valid days before value")]
    BadValidDaysBefore,
    #[error("bad valid days after value")]
    BadValidDaysAfter,
    #[error("bad subject alt name value")]
    BadSubjectAltName,
    #[error("bad key")]
    BadKey,
    #[error("unable to create a certificate builder")]
    Builder(x509_cert::builder::Error),
    #[error("unable to add a subject alt name extension")]
    SubjectAltNameExtension(x509_cert::builder::Error),
    #[error("unable to build the certificate")]
    Build(x509_cert::builder::Error),
}

impl Params<'_> {
    /// Generate a new self-signed certificate from this params and
    /// a given key.
    ///
    /// A self signed certificate is a certificate containing the public
    /// component of a key and signed by the same very key.
    ///
    /// You can use any key that works with [`signature`] and
    /// [`x509_cert::spki`] crates.
    ///
    /// # Example
    ///
    /// ```
    /// let params = xwt_cert_gen::Params {
    ///   common_name: "My certificate",
    ///   subject_alt_names: &["localhost", "127.0.0.1", "::1"],
    ///   valid_days_before: 0,
    ///   valid_days_after: 14,
    /// };
    ///
    /// // Using `p256` crate to create a NIST P-256 secret key.
    /// let key = p256::SecretKey::random(&mut rand::thread_rng());
    ///
    /// // Prepare ECDSA/P-256 signing key.
    /// let key = p256::ecdsa::SigningKey::from(key);
    ///
    /// // Create a new self-signed certificate - a certificate containing
    /// // the public component of this key and signed by this same very key.
    /// let cert = params
    ///   .self_signed::<_, p256::ecdsa::DerSignature>(&key)
    ///   .unwrap();
    /// ```
    pub fn self_signed<Key, Signature>(&self, key: &Key) -> Result<x509_cert::Certificate, Error>
    where
        Key: x509_cert::spki::DynSignatureAlgorithmIdentifier,
        Key: signature::Keypair,
        Key::VerifyingKey: x509_cert::spki::EncodePublicKey,
        Key: signature::Signer<Signature>,
        Signature: x509_cert::spki::SignatureBitStringEncoding,
    {
        use x509_cert::builder::Builder;

        let now = std::time::SystemTime::now();

        let subject: x509_cert::name::Name = format!("CN={}", self.common_name)
            .parse()
            .map_err(Error::BadCommonName)?;

        let profile = x509_cert::builder::Profile::Leaf {
            issuer: subject.clone(),
            enable_key_agreement: true,
            enable_key_encipherment: true,
        };
        let serial_number = x509_cert::serial_number::SerialNumber::from(1u8);
        let validity = x509_cert::time::Validity {
            not_before: (now - (self.valid_days_before * DAY))
                .try_into()
                .map_err(|_| Error::BadValidDaysBefore)?,
            not_after: (now + (self.valid_days_after * DAY))
                .try_into()
                .map_err(|_| Error::BadValidDaysAfter)?,
        };

        let subject_public_key_info =
            x509_cert::spki::SubjectPublicKeyInfoOwned::from_key(key.verifying_key())
                .map_err(|_| Error::BadKey)?;

        let mut builder = x509_cert::builder::CertificateBuilder::new(
            profile,
            serial_number,
            validity,
            subject,
            subject_public_key_info,
            key,
        )
        .map_err(Error::Builder)?;

        let parse_subject_alt_name =
            |s: &str| -> Result<x509_cert::ext::pkix::name::GeneralName, Error> {
                Ok(match s.parse::<core::net::IpAddr>() {
                    Ok(ip) => x509_cert::ext::pkix::name::GeneralName::from(ip),
                    Err(_) => {
                        let val =
                            der::asn1::Ia5String::new(s).map_err(|_| Error::BadSubjectAltName)?;
                        x509_cert::ext::pkix::name::GeneralName::DnsName(val)
                    }
                })
            };

        let subject_alt_names = self
            .subject_alt_names
            .iter()
            .copied()
            .map(parse_subject_alt_name)
            .collect::<Result<_, _>>()?;

        builder
            .add_extension(&x509_cert::ext::pkix::SubjectAltName(subject_alt_names))
            .map_err(Error::SubjectAltNameExtension)?;

        let cert = builder.build().map_err(Error::Builder)?;
        Ok(cert)
    }
}

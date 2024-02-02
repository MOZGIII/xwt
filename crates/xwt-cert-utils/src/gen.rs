//! Certificate generation facilities.

/// The params for certificate generation.
#[allow(missing_docs)]
pub struct Params<'a> {
    pub common_name: &'a str,
    pub subject_alt_names: &'a [&'a str],
    pub valid_days_before: u32,
    pub valid_days_after: u32,
}

#[cfg(feature = "rcgen")]
impl<'a> Params<'a> {
    /// Convert params into [`rcgen::CertificateParams`].
    pub fn into_rcgen_params(
        self,
        key_alg: &'static rcgen::SignatureAlgorithm,
        key_pair: rcgen::KeyPair,
    ) -> rcgen::CertificateParams {
        let mut dname = rcgen::DistinguishedName::new();
        dname.push(rcgen::DnType::CommonName, self.common_name);

        let now = time::OffsetDateTime::now_utc();

        let mut cert_params = rcgen::CertificateParams::default();

        cert_params
            .distinguished_name
            .push(rcgen::DnType::CommonName, self.common_name);
        cert_params
            .subject_alt_names
            .extend(self.subject_alt_names.iter().map(|&s| match s.parse() {
                Ok(ip) => rcgen::SanType::IpAddress(ip),
                Err(_) => rcgen::SanType::DnsName(s.to_owned()),
            }));
        cert_params.alg = key_alg;
        cert_params.key_pair = Some(key_pair);
        cert_params.not_before = now
            .checked_sub(time::Duration::days(self.valid_days_before.into()))
            .unwrap();
        cert_params.not_after = now
            .checked_add(time::Duration::days(self.valid_days_after.into()))
            .unwrap();

        cert_params
    }

    /// Convert params into [`rcgen::Certificate`].
    pub fn into_rcgen_cert(
        self,
        key_alg: &'static rcgen::SignatureAlgorithm,
        key_pair: rcgen::KeyPair,
    ) -> Result<rcgen::Certificate, rcgen::Error> {
        rcgen::Certificate::from_params(self.into_rcgen_params(key_alg, key_pair))
    }
}

//! High-level API for configuring the transport.

use crate::sys;

/// Options for configuring the transport.
///
/// See <https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/WebTransport#options>.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WebTransportOptions {
    /// If `true``, the network connection for this WebTransport can be shared
    /// with a pool of other HTTP/3 sessions.
    ///
    /// By default the value is false, and the connection cannot be shared.
    pub allow_pooling: bool,
    /// Indicates the application's preference that the congestion control
    /// algorithm used when sending data over this connection be tuned for
    /// either throughput or low-latency.
    ///
    /// his is a hint to the user agent.
    pub congestion_control: CongestionControl,
    /// If true, the connection cannot be established over HTTP/2 if an HTTP/3
    /// connection is not possible.
    ///
    /// By default the value is false.
    pub require_unreliable: bool,
    /// An array of objects, each defining the hash value of a server
    /// certificate along with the name of the algorithm that was used to
    /// generate it.
    ///
    /// This option is only supported for transports using dedicated connections
    /// (`allow_pooling` is false).
    ///
    /// If specified, the browser will attempt to authenticate the certificate
    /// provided by the server against the provided certificate hash(es) in
    /// order to connect, instead of using the Web public key infrastructure
    /// (PKI). If any hashes match, the browser knows that the server has
    /// possession of a trusted certificate and will connect as normal. If empty
    /// the user agent uses the same PKI certificate verification procedures it
    /// would use for a normal fetch operation.
    ///
    /// This feature allows developers to connect to WebTransport servers that
    /// would normally find obtaining a publicly trusted certificate
    /// challenging, such as hosts that are not publicly routable, or ephemeral
    /// hosts like virtual machines.
    pub server_certificate_hashes: Vec<CertificateHash>,
}

/// The application's preference that the congestion control algorithm used when
/// sending data over this connection be tuned for either throughput or
/// low-latency.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum CongestionControl {
    /// Default value.
    #[default]
    Default,
    /// Favour throughput.
    Throughput,
    /// Favour low latency.
    LowLatency,
}

/// Hash of a server certificate which the transport can connect to.
///
/// The certificate must be an X.509v3 certificate that has a validity period of
/// less that 2 weeks, and the current time must be within that validity period.
/// The format of the public key in the certificate depends on the
/// implementation, but must minimally include ECDSA with the secp256r1 (NIST
/// P-256) named group, and must not include RSA keys. An ECSDA key is therefore
/// an interoperable default public key format. A user agent may add further
/// requirements; these will be listed in the [browser compatibility] section if
/// known.
///
/// See <https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/WebTransport#servercertificatehashes>.
///
/// [browser compatibility]: https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/WebTransport#browser_compatibility
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CertificateHash {
    /// Algorithm used to verify the hash.
    pub algorithm: HashAlgorithm,
    /// Hash value.
    pub value: Vec<u8>,
}

/// Algorithm used to verify the hash.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HashAlgorithm {
    /// SHA-256 algorithm.
    Sha256,
}

impl WebTransportOptions {
    /// Creates a JavaScript value from this value.
    pub fn to_js(&self) -> sys::WebTransportOptions {
        let mut js = sys::WebTransportOptions::new();
        js.allow_pooling(self.allow_pooling);
        js.congestion_control(match self.congestion_control {
            CongestionControl::Default => sys::WebTransportCongestionControl::Default,
            CongestionControl::Throughput => sys::WebTransportCongestionControl::Throughput,
            CongestionControl::LowLatency => sys::WebTransportCongestionControl::LowLatency,
        });
        js.require_unreliable(self.require_unreliable);

        let cert_hashes = self
            .server_certificate_hashes
            .iter()
            .map(|cert| {
                let mut hash = sys::WebTransportHash::new();
                hash.algorithm(match cert.algorithm {
                    HashAlgorithm::Sha256 => "sha-256",
                });
                hash.value(&js_sys::Uint8Array::from(cert.value.as_ref()));
                wasm_bindgen::JsValue::from(hash)
            })
            .collect::<js_sys::Array>();
        js.server_certificate_hashes(&cert_hashes);

        js
    }
}

//! The xwt test assets.

/// The certificate in the DER X.509 format.
pub const CERT: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/target/cert.der"));

/// The private key in the DER PKCS 8 format.
pub const KEY: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/target/key-pkcs8.der"));

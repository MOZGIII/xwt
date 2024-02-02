//! The xwt test assets.

/// The certificate in the DER format.
pub const CERT: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/cert.der"));

/// The private key in the DER format.
pub const KEY: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/key.der"));

/// Compute a SHA256 digest for the given data.
///
/// Pass DER certificate here to compute its digest.
#[cfg(feature = "digest-sha256")]
pub fn sha256(data: &[u8]) -> ring::digest::Digest {
    ring::digest::digest(&ring::digest::SHA256, data)
}

//! Conversions of the WebTransport error codes from and to the HTTP3 error
//! codes.
//!
//! We need this because [`wtransport`] does not convert error codes at all.

/// Lower end of the WebTransport HTTP codes.
const FIRST: u64 = 0x52e4a40fa8db;
/// Higher end of the WebTransport HTTP codes.
const LAST: u64 = 0x52e5ac983162;

/// Convert the WebTransport code value to HTTP3 code.
pub fn to_http(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }

    let n: u64 = n.into();
    FIRST + n + (n / 0x1e)
}

/// An error that occurs when converting the HTTP error code to
/// WebTransport error code.
#[derive(Debug, thiserror::Error)]
#[error("unable to convert HTTP error code to WebTransport error code: {0}")]
pub struct FromHttpError(u64);

/// Convert the HTTP3 code value to WebTransport code.
pub fn from_http(h: u64) -> Result<u32, FromHttpError> {
    if h == 0 {
        return Ok(0);
    }

    #[allow(clippy::manual_is_multiple_of)]
    if !((FIRST..=LAST).contains(&h) && (h - 0x21) % 0x1f != 0) {
        return Err(FromHttpError(h));
    }
    let shifted = h - FIRST;
    let val = shifted - (shifted / 0x1f);
    Ok(val.try_into().unwrap())
}

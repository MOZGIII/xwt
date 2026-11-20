//! Conversions of the WebTransport error codes from and to the HTTP3 error
//! codes.
//!
//! We need this because [`wtransport`] does not convert error codes at all.

/// Lower end of the WebTransport HTTP codes.
const FIRST: u64 = 0x52e4a40fa8db;
/// Higher end of the WebTransport HTTP codes.
const LAST: u64 = 0x52e5ac983162;

/// Convert the WebTransport code value to HTTP3 code.
///
/// Every code, `0` included, maps into the WebTransport application range,
/// as the spec requires; peers report a code outside that range as having no
/// application error code at all.
pub fn to_http(n: u32) -> u64 {
    let n: u64 = n.into();
    FIRST + n + (n / 0x1e)
}

/// An error that occurs when converting the HTTP error code to
/// WebTransport error code.
#[derive(Debug, thiserror::Error)]
#[error("unable to convert HTTP error code to WebTransport error code: {0}")]
pub struct FromHttpError(u64);

/// Convert the HTTP3 code value to WebTransport code.
///
/// Codes outside the WebTransport application range - the raw `0` included -
/// carry no WebTransport code and are rejected.
pub fn from_http(h: u64) -> Result<u32, FromHttpError> {
    #[allow(clippy::manual_is_multiple_of)]
    if !((FIRST..=LAST).contains(&h) && (h - 0x21) % 0x1f != 0) {
        return Err(FromHttpError(h));
    }
    let shifted = h - FIRST;
    let val = shifted - (shifted / 0x1f);
    Ok(val.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_maps_into_the_application_range() {
        assert_eq!(to_http(0), FIRST);
        assert_eq!(from_http(FIRST).unwrap(), 0);
    }

    #[test]
    fn raw_zero_is_rejected() {
        assert!(from_http(0).is_err());
    }

    #[test]
    fn round_trip() {
        for n in [0, 1, 0x1d, 0x1e, 0x1f, 123, 0xffff, u32::MAX] {
            let h = to_http(n);
            assert!((FIRST..=LAST).contains(&h), "{n} maps to {h:#x}");
            assert_eq!(from_http(h).unwrap(), n);
        }
    }
}

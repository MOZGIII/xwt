//! [`xwt_core::stream::ErrorAsErrorCode`] implementations.

use wasm_bindgen::JsCast as _;

use crate::{StreamReadError, StreamWriteError};

impl xwt_core::stream::ErrorAsErrorCode for StreamReadError {
    fn as_error_code(&self) -> Option<xwt_core::stream::ErrorCode> {
        let error = match self {
            Self::ByobReadConsumedBuffer => return None,
            Self::Closed => return Some(0),
            Self::Read(error) => error,
        };

        let error: &web_wt_sys::WebTransportError = error.0.dyn_ref()?;
        if error.source() != web_wt_sys::WebTransportErrorSource::Stream {
            return None;
        }
        let code = error.stream_error_code()?;
        Some(code)
    }
}

// TODO: verify this implementation
impl xwt_core::stream::ErrorAsErrorCode for StreamWriteError {
    fn as_error_code(&self) -> Option<xwt_core::stream::ErrorCode> {
        let error = match self {
            Self::ZeroSizeWriteBuffer => return None,
            Self::Write(error) => error,
        };

        let error: &web_wt_sys::WebTransportError = error.0.dyn_ref()?;
        if error.source() != web_wt_sys::WebTransportErrorSource::Stream {
            return None;
        }
        let code = error.stream_error_code()?;
        Some(code)
    }
}

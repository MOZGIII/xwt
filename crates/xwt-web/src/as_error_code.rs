//! [`xwt_core::stream::AsErrorCode`] implementations.

use wasm_bindgen::JsCast as _;

use crate::{StreamErrorCode, StreamReadError, StreamWriteError};

impl xwt_core::stream::AsErrorCode for StreamReadError {
    type ErrorCode = StreamErrorCode;

    fn as_error_code(&self) -> Option<StreamErrorCode> {
        let error = match self {
            Self::ByobReadConsumedBuffer => return None,
            Self::Closed => return Some(0.into()),
            Self::Read(error) => error,
        };

        let error: &web_wt_sys::WebTransportError = error.0.dyn_ref()?;
        if error.source() != web_wt_sys::WebTransportErrorSource::Stream {
            return None;
        }
        let code = error.stream_error_code().unwrap_or(0);
        Some(code.into())
    }
}

// TODO: verify this implementation
impl xwt_core::stream::AsErrorCode for StreamWriteError {
    type ErrorCode = StreamErrorCode;

    fn as_error_code(&self) -> Option<StreamErrorCode> {
        let error = match self {
            Self::ZeroSizeWriteBuffer => return None,
            Self::Write(error) => error,
        };

        let error: &web_wt_sys::WebTransportError = error.0.dyn_ref()?;
        if error.source() != web_wt_sys::WebTransportErrorSource::Stream {
            return None;
        }
        let code = error.stream_error_code().unwrap_or(0);
        Some(code.into())
    }
}

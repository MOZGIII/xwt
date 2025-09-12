//! [`xwt_core::stream::ErrorAsErrorCode`] implementations.

use wasm_bindgen::JsCast as _;

use crate::{Error, StreamReadError, StreamWriteError};

impl xwt_core::stream::ErrorAsErrorCode for Error {
    fn as_error_code(&self) -> Option<xwt_core::stream::ErrorCode> {
        let error: &web_wt_sys::WebTransportError = self.0.dyn_ref()?;
        if error.source() != web_wt_sys::WebTransportErrorSource::Stream {
            return None;
        }
        let code = error.stream_error_code()?;
        Some(code)
    }
}
impl xwt_core::stream::ErrorAsErrorCode for StreamReadError {
    fn as_error_code(&self) -> Option<xwt_core::stream::ErrorCode> {
        match self {
            Self::ByobReadConsumedBuffer => None,
            Self::Closed => Some(0),
            Self::Read(error) => error.as_error_code(),
        }
    }
}

impl xwt_core::stream::ErrorAsErrorCode for StreamWriteError {
    fn as_error_code(&self) -> Option<xwt_core::stream::ErrorCode> {
        match self {
            Self::ZeroSizeWriteBuffer => None,
            Self::Write(error) => error.as_error_code(),
        }
    }
}

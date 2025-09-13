//! [`xwt_core::stream::ErrorAsErrorCode`] implementations.

use crate::{error_codes, StreamFinishedError, StreamReadError, StreamWriteError};

impl xwt_core::stream::ErrorAsErrorCode for StreamReadError {
    fn as_error_code(&self) -> Option<xwt_core::stream::ErrorCode> {
        match self {
            Self::Closed => Some(0),

            Self::Read(wtransport::error::StreamReadError::Reset(error_code)) => {
                let code = error_codes::from_http(error_code.into_inner()).ok()?;
                Some(code)
            }

            Self::Read(_) => None,

            Self::ZeroBytes => None,
        }
    }
}

impl xwt_core::stream::ErrorAsErrorCode for StreamWriteError {
    fn as_error_code(&self) -> Option<xwt_core::stream::ErrorCode> {
        match self {
            Self::ZeroSizeWriteBuffer => None,
            Self::Write(wtransport::error::StreamWriteError::Closed) => Some(0),

            Self::Write(wtransport::error::StreamWriteError::Stopped(error_code)) => {
                let code = error_codes::from_http(error_code.into_inner()).ok()?;
                Some(code)
            }

            Self::Write(_) => None,
        }
    }
}

impl xwt_core::stream::ErrorAsErrorCode for StreamFinishedError {
    fn as_error_code(&self) -> Option<xwt_core::stream::ErrorCode> {
        match self {
            Self::Read(wtransport::error::StreamReadError::Reset(error_code)) => {
                let code = error_codes::from_http(error_code.into_inner()).ok()?;
                Some(code)
            }

            Self::Read(_) => None,

            Self::TrailingData { .. } => None,
        }
    }
}

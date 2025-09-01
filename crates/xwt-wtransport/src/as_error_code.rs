//! [`xwt_core::stream::AsErrorCode`] implementations.

use crate::{error_codes, StreamErrorCode, StreamReadError, StreamWriteError};

impl xwt_core::stream::AsErrorCode for StreamReadError {
    type ErrorCode = StreamErrorCode;

    fn as_error_code(&self) -> Option<StreamErrorCode> {
        match self {
            Self::Closed => Some(0.into()),

            Self::Read(wtransport::error::StreamReadError::Reset(error_code)) => {
                let code = error_codes::from_http(error_code.into_inner()).ok()?;
                Some(code.into())
            }

            Self::Read(_) => None,
        }
    }
}

impl xwt_core::stream::AsErrorCode for StreamWriteError {
    type ErrorCode = StreamErrorCode;

    fn as_error_code(&self) -> Option<StreamErrorCode> {
        match self {
            Self::ZeroSizeWriteBuffer => None,
            Self::Write(wtransport::error::StreamWriteError::Closed) => Some(0.into()),

            Self::Write(wtransport::error::StreamWriteError::Stopped(error_code)) => {
                let code = error_codes::from_http(error_code.into_inner()).ok()?;
                Some(code.into())
            }

            Self::Write(_) => None,
        }
    }
}

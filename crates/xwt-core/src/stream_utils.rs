//! Utilitites for the WebTransport streams.

use crate::stream;

/// A shortcut for the error type for a given [`stream::Read`] type.
pub type ReadErrorFor<T> = <T as stream::Read>::Error;

/// A shortcut for the error type for a given [`stream::Write`] type.
pub type WriteErrorFor<T> = <T as stream::Write>::Error;

/// A shortcut for the error type for a given [`stream::ReadChunk] type.
pub type ReadChunkErrorFor<T, Data> = <T as stream::ReadChunk<Data>>::Error;

/// A shortcut for the error type for a given [`stream::WriteChunk] type.
pub type WriteChunkErrorFor<T, Data> = <T as stream::WriteChunk<Data>>::Error;

/// A shortcut for the error type for a given [`stream::ReadAbort`] type.
pub type ReadAbortErrorFor<T> = <T as stream::ReadAbort>::Error;

/// A shortcut for the error type for a given [`stream::WriteAbort`] type.
pub type WriteAbortErrorFor<T> = <T as stream::WriteAbort>::Error;

/// A shortcut for the error type for a given [`stream::WriteAborted`] type.
pub type WriteAbortedErrorFor<T> = <T as stream::WriteAborted>::Error;

/// A shortcut for the error type for a given [`stream::Finish`] type.
pub type FinishErrorFor<T> = <T as stream::Finish>::Error;

/// Extensions to the [`stream::ErrorAsErrorCode`] providing some convenience methods.
pub trait ErrorAsErrorCodeExt: stream::ErrorAsErrorCode {
    /// Check of the error code matches the given value.
    fn is_error_code(&self, expected_code: stream::ErrorCode) -> bool;

    /// Checks that the error code exists and is zero.
    fn is_closed(&self) -> bool;
}

// Check that this trait is dyn-safe.
impl dyn ErrorAsErrorCodeExt {}

impl<T> ErrorAsErrorCodeExt for T
where
    T: ?Sized + stream::ErrorAsErrorCode,
{
    fn is_error_code(&self, expected_error_code: stream::ErrorCode) -> bool {
        let Some(error_code) = self.as_error_code() else {
            return false;
        };

        error_code == expected_error_code
    }

    fn is_closed(&self) -> bool {
        self.is_error_code(0)
    }
}

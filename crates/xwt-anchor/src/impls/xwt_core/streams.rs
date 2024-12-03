//! Implementations related to streams.

use core::num::NonZeroUsize;

use crate::types::*;

impl<T> xwt_core::stream::Write for SendStream<T>
where
    T: xwt_core::stream::Write,
{
    type ErrorCode = T::ErrorCode;
    type Error = T::Error;

    async fn write(&mut self, buf: &[u8]) -> Result<NonZeroUsize, Self::Error> {
        T::write(&mut self.0, buf).await
    }
}

impl<T> xwt_core::stream::WriteAbort for SendStream<T>
where
    T: xwt_core::stream::WriteAbort,
{
    type ErrorCode = T::ErrorCode;
    type Error = T::Error;

    async fn abort(self, error_code: Self::ErrorCode) -> Result<(), Self::Error> {
        T::abort(self.0, error_code).await
    }
}

impl<T> xwt_core::stream::WriteAborted for SendStream<T>
where
    T: xwt_core::stream::WriteAborted,
{
    type ErrorCode = T::ErrorCode;
    type Error = T::Error;

    async fn aborted(self) -> Result<Self::ErrorCode, Self::Error> {
        T::aborted(self.0).await
    }
}

impl<T> xwt_core::stream::Finish for SendStream<T>
where
    T: xwt_core::stream::Finish,
{
    type Error = T::Error;

    async fn finish(self) -> Result<(), Self::Error> {
        T::finish(self.0).await
    }
}

impl<T> xwt_core::stream::Read for RecvStream<T>
where
    T: xwt_core::stream::Read,
{
    type ErrorCode = T::ErrorCode;
    type Error = T::Error;

    async fn read(&mut self, buf: &mut [u8]) -> Result<NonZeroUsize, Self::Error> {
        T::read(&mut self.0, buf).await
    }
}

impl<T> xwt_core::stream::ReadAbort for RecvStream<T>
where
    T: xwt_core::stream::ReadAbort,
{
    type ErrorCode = T::ErrorCode;
    type Error = T::Error;

    async fn abort(self, error_code: Self::ErrorCode) -> Result<(), Self::Error> {
        T::abort(self.0, error_code).await
    }
}

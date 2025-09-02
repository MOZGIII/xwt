//! Implementations related to streams.

use core::num::NonZeroUsize;

use crate::types::*;

impl<T> xwt_core::stream::Write for SendStream<T>
where
    T: xwt_core::stream::Write,
{
    type Error = T::Error;

    async fn write(&mut self, buf: &[u8]) -> Result<NonZeroUsize, Self::Error> {
        T::write(&mut self.0, buf).await
    }
}

impl<T> xwt_core::stream::WriteAbort for SendStream<T>
where
    T: xwt_core::stream::WriteAbort,
{
    type Error = T::Error;

    async fn abort(self, error_code: xwt_core::stream::ErrorCode) -> Result<(), Self::Error> {
        T::abort(self.0, error_code).await
    }
}

impl<T> xwt_core::stream::WriteAborted for SendStream<T>
where
    T: xwt_core::stream::WriteAborted,
{
    type Error = T::Error;

    async fn aborted(self) -> Result<xwt_core::stream::ErrorCode, Self::Error> {
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
    type Error = T::Error;

    async fn read(&mut self, buf: &mut [u8]) -> Result<NonZeroUsize, Self::Error> {
        T::read(&mut self.0, buf).await
    }
}

impl<T> xwt_core::stream::ReadAbort for RecvStream<T>
where
    T: xwt_core::stream::ReadAbort,
{
    type Error = T::Error;

    async fn abort(self, error_code: xwt_core::stream::ErrorCode) -> Result<(), Self::Error> {
        T::abort(self.0, error_code).await
    }
}

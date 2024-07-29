//! Implementations related to streams.

use crate::types::*;

impl<T> xwt_core::stream::Write for SendStream<T>
where
    T: xwt_core::stream::Write,
{
    type Error = T::Error;

    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        T::write(&mut self.0, buf).await
    }
}

impl<T> xwt_core::stream::Read for RecvStream<T>
where
    T: xwt_core::stream::Read,
{
    type Error = T::Error;

    async fn read(&mut self, buf: &mut [u8]) -> Result<Option<usize>, Self::Error> {
        T::read(&mut self.0, buf).await
    }
}

//! Implementations related to connection stream operations.

use xwt_core::utils::maybe;

use crate::types::*;

impl<T> xwt_core::Streams for Connection<T>
where
    T: xwt_core::Streams,
{
    type SendStream = SendStream<T::SendStream>;
    type RecvStream = RecvStream<T::RecvStream>;
}

impl<T> xwt_core::Streams for Streams<T>
where
    T: xwt_core::Streams,
{
    type SendStream = SendStream<T::SendStream>;
    type RecvStream = RecvStream<T::RecvStream>;
}

impl<T> xwt_core::OpenBiStream for Connection<T>
where
    T: xwt_core::OpenBiStream + maybe::Send + maybe::Sync,
{
    type Opening = OpeningBiStream<T::Opening>;
    type Error = T::Error;

    async fn open_bi(&self) -> Result<Self::Opening, Self::Error> {
        T::open_bi(&self.0).await.map(OpeningBiStream)
    }
}

impl<T> xwt_core::OpeningBiStream for OpeningBiStream<T>
where
    T: xwt_core::OpeningBiStream,
{
    type Streams = Connection<T::Streams>;
    type Error = T::Error;

    async fn wait_bi(self) -> Result<xwt_core::BiStreamsFor<Self::Streams>, Self::Error> {
        T::wait_bi(self.0)
            .await
            .map(|(send, recv)| (SendStream(send), RecvStream(recv)))
    }
}

impl<T> xwt_core::OpenUniStream for Connection<T>
where
    T: xwt_core::OpenUniStream + maybe::Send + maybe::Sync,
{
    type Opening = OpeningUniStream<T::Opening>;
    type Error = T::Error;

    async fn open_uni(&self) -> Result<Self::Opening, Self::Error> {
        T::open_uni(&self.0).await.map(OpeningUniStream)
    }
}

impl<T> xwt_core::OpeningUniStream for OpeningUniStream<T>
where
    T: xwt_core::OpeningUniStream,
{
    type Streams = Streams<T::Streams>;
    type Error = T::Error;

    async fn wait_uni(
        self,
    ) -> Result<<Self::Streams as xwt_core::Streams>::SendStream, Self::Error> {
        T::wait_uni(self.0).await.map(SendStream)
    }
}

impl<T> xwt_core::AcceptBiStream for Connection<T>
where
    T: xwt_core::AcceptBiStream + maybe::Send + maybe::Sync,
{
    type Error = T::Error;

    async fn accept_bi(&self) -> Result<xwt_core::BiStreamsFor<Self>, Self::Error> {
        T::accept_bi(&self.0)
            .await
            .map(|(send, recv)| (SendStream(send), RecvStream(recv)))
    }
}

impl<T> xwt_core::AcceptUniStream for Connection<T>
where
    T: xwt_core::AcceptUniStream + maybe::Send + maybe::Sync,
{
    type Error = T::Error;

    async fn accept_uni(&self) -> Result<Self::RecvStream, Self::Error> {
        T::accept_uni(&self.0).await.map(RecvStream)
    }
}

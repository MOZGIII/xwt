//! Implementations related to session stream operations.

use xwt_core::{session::stream_utils::SendStreamFor, utils::maybe};

use crate::types::*;

impl<T> xwt_core::session::stream::SendSpec for Session<T>
where
    T: xwt_core::session::stream::SendSpec,
{
    type SendStream = SendStream<T::SendStream>;
}

impl<T> xwt_core::session::stream::RecvSpec for Session<T>
where
    T: xwt_core::session::stream::RecvSpec,
{
    type RecvStream = RecvStream<T::RecvStream>;
}

impl<T> xwt_core::session::stream::OpenBi for Session<T>
where
    T: xwt_core::session::stream::OpenBi + maybe::Send + maybe::Sync,
{
    type Opening = OpeningBiStream<T::Opening>;
    type Error = T::Error;

    async fn open_bi(&self) -> Result<Self::Opening, Self::Error> {
        T::open_bi(&self.0).await.map(OpeningBiStream)
    }
}

impl<T> xwt_core::session::stream::OpeningBi for OpeningBiStream<T>
where
    T: xwt_core::session::stream::OpeningBi,
{
    type Streams = Session<T::Streams>;
    type Error = T::Error;

    async fn wait_bi(
        self,
    ) -> Result<xwt_core::session::stream::TupleFor<Self::Streams>, Self::Error> {
        T::wait_bi(self.0)
            .await
            .map(|(send, recv)| (SendStream(send), RecvStream(recv)))
    }
}

impl<T> xwt_core::session::stream::OpenUni for Session<T>
where
    T: xwt_core::session::stream::OpenUni + maybe::Send + maybe::Sync,
{
    type Opening = OpeningUniStream<T::Opening>;
    type Error = T::Error;

    async fn open_uni(&self) -> Result<Self::Opening, Self::Error> {
        T::open_uni(&self.0).await.map(OpeningUniStream)
    }
}

impl<T> xwt_core::session::stream::OpeningUni for OpeningUniStream<T>
where
    T: xwt_core::session::stream::OpeningUni,
{
    type Streams = Session<T::Streams>;
    type Error = T::Error;

    async fn wait_uni(self) -> Result<SendStreamFor<Self::Streams>, Self::Error> {
        T::wait_uni(self.0).await.map(SendStream)
    }
}

impl<T> xwt_core::session::stream::AcceptBi for Session<T>
where
    T: xwt_core::session::stream::AcceptBi + maybe::Send + maybe::Sync,
{
    type Error = T::Error;

    async fn accept_bi(&self) -> Result<xwt_core::session::stream::TupleFor<Self>, Self::Error> {
        T::accept_bi(&self.0)
            .await
            .map(|(send, recv)| (SendStream(send), RecvStream(recv)))
    }
}

impl<T> xwt_core::session::stream::AcceptUni for Session<T>
where
    T: xwt_core::session::stream::AcceptUni + maybe::Send + maybe::Sync,
{
    type Error = T::Error;

    async fn accept_uni(&self) -> Result<Self::RecvStream, Self::Error> {
        T::accept_uni(&self.0).await.map(RecvStream)
    }
}

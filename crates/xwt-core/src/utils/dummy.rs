use crate::session::stream::{PairSpec, SendSpec};

use super::maybe;

#[derive(Debug)]
pub struct Connecting<T>(pub T);

impl<T> crate::endpoint::connect::Connecting for Connecting<T>
where
    T: maybe::Send,
{
    type Session = T;
    type Error = core::convert::Infallible;

    async fn wait_connect(self) -> Result<Self::Session, Self::Error> {
        Ok(self.0)
    }
}

#[derive(Debug)]
pub struct OpeningUniStream<T: SendSpec>(pub T::SendStream);

impl<T> crate::session::stream::OpeningUni for OpeningUniStream<T>
where
    T: SendSpec,
{
    type Streams = T;
    type Error = core::convert::Infallible;

    async fn wait_uni(self) -> Result<<T as SendSpec>::SendStream, Self::Error> {
        Ok(self.0)
    }
}

#[derive(Debug)]
pub struct OpeningBiStream<T: PairSpec>(pub (T::SendStream, T::RecvStream));

impl<T> crate::session::stream::OpeningBi for OpeningBiStream<T>
where
    T: PairSpec,
{
    type Streams = T;
    type Error = core::convert::Infallible;

    async fn wait_bi(self) -> Result<crate::session::stream::TupleFor<T>, Self::Error> {
        Ok(self.0)
    }
}

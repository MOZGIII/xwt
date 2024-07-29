//! Dummy implementations.

use crate::{
    endpoint,
    session::{
        self,
        stream::{PairSpec, SendSpec},
    },
};

use super::maybe;

/// A dummy [`Connecting`] type, that wraps a session and implements
/// [`endpoint::connect::Connecting`] trait.
///
/// Useful when implementing a driver that does not have a meaningful way
/// to encode the intermediate "connecting session" state, and provides
/// a fully connected session right away.
/// Those drivers can use this dummy type instead of implementing their own.
#[derive(Debug)]
pub struct Connecting<T>(pub T);

impl<T> endpoint::connect::Connecting for Connecting<T>
where
    T: maybe::Send,
{
    type Session = T;
    type Error = core::convert::Infallible;

    async fn wait_connect(self) -> Result<Self::Session, Self::Error> {
        Ok(self.0)
    }
}

/// A dummy [`OpeningUniStream`] type, that wraps a send stream and implements
/// [`session::stream::OpeningUni`] trait.
///
/// Useful when implementing a driver that does not have a meaningful way
/// to encode the intermediate "opening unidirectional stream" state, and
/// provides a fully opened stream right away.
/// Those drivers can use this dummy type instead of implementing their own.
#[derive(Debug)]
pub struct OpeningUniStream<T: SendSpec>(pub T::SendStream);

impl<T> session::stream::OpeningUni for OpeningUniStream<T>
where
    T: SendSpec,
{
    type Streams = T;
    type Error = core::convert::Infallible;

    async fn wait_uni(self) -> Result<<T as SendSpec>::SendStream, Self::Error> {
        Ok(self.0)
    }
}

/// A dummy [`OpeningBiStream`] type, that wraps a pair of streams and
/// implements [`session::stream::OpeningBi`] trait.
///
/// Useful when implementing a driver that does not have a meaningful way
/// to encode the intermediate "opening bidirectional stream" state, and
/// provides a pair of fully opened streams right away.
/// Those drivers can use this dummy type instead of implementing their own.
#[derive(Debug)]
pub struct OpeningBiStream<T: PairSpec>(pub (T::SendStream, T::RecvStream));

impl<T> session::stream::OpeningBi for OpeningBiStream<T>
where
    T: PairSpec,
{
    type Streams = T;
    type Error = core::convert::Infallible;

    async fn wait_bi(self) -> Result<session::stream::TupleFor<T>, Self::Error> {
        Ok(self.0)
    }
}

#![allow(missing_docs)]

use core::future::Future;

use crate::utils::{maybe, Error};

pub trait SendSpec: maybe::Send {
    type SendStream: crate::stream::Write
        + crate::stream::WriteAbort
        + crate::stream::WriteAborted
        + crate::stream::Finish;
}

pub trait RecvSpec: maybe::Send {
    type RecvStream: crate::stream::Read
        + crate::stream::ReadAbort
        + crate::stream::ReadAborted
        + crate::stream::Finished;
}

pub trait PairSpec: maybe::Send + SendSpec + RecvSpec {}

impl<T> PairSpec for T where T: maybe::Send + SendSpec + RecvSpec {}

pub type TupleFor<T> = (<T as SendSpec>::SendStream, <T as RecvSpec>::RecvStream);

pub trait OpeningBi: maybe::Send {
    type Streams: PairSpec;
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn wait_bi(
        self,
    ) -> impl Future<Output = Result<TupleFor<Self::Streams>, Self::Error>> + maybe::Send;
}

pub trait OpenBi: PairSpec {
    type Opening: OpeningBi<Streams = Self>;
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn open_bi(&self) -> impl Future<Output = Result<Self::Opening, Self::Error>> + maybe::Send;
}

pub trait AcceptBi: PairSpec {
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn accept_bi(&self) -> impl Future<Output = Result<TupleFor<Self>, Self::Error>> + maybe::Send;
}

pub trait OpeningUni: maybe::Send {
    type Streams: SendSpec;
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn wait_uni(
        self,
    ) -> impl Future<Output = Result<<Self::Streams as SendSpec>::SendStream, Self::Error>> + maybe::Send;
}

pub trait OpenUni: SendSpec {
    type Opening: OpeningUni<Streams = Self>;
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn open_uni(&self) -> impl Future<Output = Result<Self::Opening, Self::Error>> + maybe::Send;
}

pub trait AcceptUni: RecvSpec {
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn accept_uni(
        &self,
    ) -> impl Future<Output = Result<Self::RecvStream, Self::Error>> + maybe::Send;
}

use core::future::Future;

use crate::{
    datagram,
    io::{Read, Write},
    utils::{maybe, Error},
};

pub trait Streams: maybe::Send {
    type SendStream: Write;
    type RecvStream: Read;
}

pub type BiStreamsFor<T> = (<T as Streams>::SendStream, <T as Streams>::RecvStream);

pub trait OpeningBiStream: maybe::Send {
    type Streams: Streams;
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn wait_bi(
        self,
    ) -> impl Future<Output = Result<BiStreamsFor<Self::Streams>, Self::Error>> + maybe::Send;
}

pub trait OpenBiStream: Streams {
    type Opening: OpeningBiStream<Streams = Self>;
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn open_bi(&self) -> impl Future<Output = Result<Self::Opening, Self::Error>> + maybe::Send;
}

pub trait AcceptBiStream: Streams {
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn accept_bi(
        &self,
    ) -> impl Future<Output = Result<BiStreamsFor<Self>, Self::Error>> + maybe::Send;
}

pub trait OpeningUniStream: maybe::Send {
    type Streams: Streams;
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn wait_uni(
        self,
    ) -> impl Future<Output = Result<<Self::Streams as Streams>::SendStream, Self::Error>> + maybe::Send;
}

pub trait OpenUniStream: Streams {
    type Opening: OpeningUniStream;
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn open_uni(&self) -> impl Future<Output = Result<Self::Opening, Self::Error>> + maybe::Send;
}

pub trait AcceptUniStream: Streams {
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn accept_uni(
        &self,
    ) -> impl Future<Output = Result<Self::RecvStream, Self::Error>> + maybe::Send;
}

pub trait EndpointConnect: Sized + maybe::Send {
    type Connecting: Connecting;
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn connect(
        &self,
        url: &str,
    ) -> impl Future<Output = Result<Self::Connecting, Self::Error>> + maybe::Send;
}

pub trait Connecting: maybe::Send {
    type Connection: maybe::Send;
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn wait_connect(
        self,
    ) -> impl Future<Output = Result<Self::Connection, Self::Error>> + maybe::Send;
}

pub trait EndpointAccept: Sized + maybe::Send {
    type Accepting: Accepting;
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn accept(
        &self,
    ) -> impl Future<Output = Result<Option<Self::Accepting>, Self::Error>> + maybe::Send;
}

pub trait Accepting: maybe::Send {
    type Request: Request;
    type Error: Error + maybe::Send + maybe::Sync + 'static;

    fn wait_accept(self) -> impl Future<Output = Result<Self::Request, Self::Error>> + maybe::Send;
}

pub trait Request: maybe::Send {
    type Connection: maybe::Send;
    type OkError: Error + maybe::Send + maybe::Sync + 'static;
    type CloseError: Error + maybe::Send + maybe::Sync + 'static;

    fn ok(self) -> impl Future<Output = Result<Self::Connection, Self::OkError>> + maybe::Send;

    fn close(self, status: u16)
        -> impl Future<Output = Result<(), Self::CloseError>> + maybe::Send;
}

pub trait Connection:
    Streams + OpenBiStream + OpenUniStream + AcceptBiStream + AcceptUniStream + datagram::Datagrams
{
}

impl<T> Connection for T where
    T: Streams
        + OpenBiStream
        + OpenUniStream
        + AcceptBiStream
        + AcceptUniStream
        + datagram::Datagrams
{
}

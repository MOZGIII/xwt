use crate::utils::maybe;

pub trait Streams {
    type SendStream;
    type RecvStream;
}

pub trait OpeningBiStream {
    type Streams: Streams;
    type Error: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn wait(
        self,
    ) -> Result<
        (
            <Self::Streams as Streams>::SendStream,
            <Self::Streams as Streams>::RecvStream,
        ),
        Self::Error,
    >;
}

pub trait OpenBiStream: Streams {
    type Opening: OpeningBiStream<Streams = Self>;
    type Error: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn open_bi(&self) -> Result<Self::Opening, Self::Error>;
}

pub trait AcceptBiStream: Streams {
    type Error: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn accept_bi(&self) -> Result<(Self::SendStream, Self::RecvStream), Self::Error>;
}

pub trait OpeningUniStream {
    type Streams: Streams;
    type Error: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn wait(self) -> Result<<Self::Streams as Streams>::SendStream, Self::Error>;
}

pub trait OpenUniStream: Streams {
    type Opening: OpeningUniStream;
    type Error: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn open_uni(&self) -> Result<Self::Opening, Self::Error>;
}

pub trait AcceptUniStream: Streams {
    type Error: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn accept_uni(&self) -> Result<Self::RecvStream, Self::Error>;
}

pub trait Connecting {
    type Connection;
    type Error: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn wait(self) -> Result<Self::Connection, Self::Error>;
}

pub trait EndpointConnect: Sized {
    type Params<'params>;
    type Connecting: Connecting;
    type ConnectError: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn connect(
        &self,
        url: &str,
        params: Self::Params<'_>,
    ) -> Result<Self::Connecting, Self::ConnectError>;
}

pub trait EndpointAccept: Sized {
    type Connecting: Connecting;
    type AcceptError: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn accept(&self) -> Result<Self::Connecting, Self::AcceptError>;
}

pub trait Connection:
    Streams + OpenBiStream + OpenUniStream + AcceptBiStream + AcceptUniStream
{
}

impl<T> Connection for T where
    T: Streams + OpenBiStream + OpenUniStream + AcceptBiStream + AcceptUniStream
{
}

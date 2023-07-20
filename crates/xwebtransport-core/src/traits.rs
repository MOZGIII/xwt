use async_trait::async_trait;

use crate::utils::maybe;

pub trait Streams: maybe::Send {
    type SendStream: maybe::Send + tokio::io::AsyncWrite;
    type RecvStream: maybe::Send + tokio::io::AsyncRead;
}

pub type BiStreamsFor<T> = (<T as Streams>::SendStream, <T as Streams>::RecvStream);

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait OpeningBiStream: maybe::Send {
    type Streams: Streams;
    type Error: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn wait_bi(self) -> Result<BiStreamsFor<Self::Streams>, Self::Error>;
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait OpenBiStream: Streams {
    type Opening: OpeningBiStream<Streams = Self>;
    type Error: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn open_bi(&self) -> Result<Self::Opening, Self::Error>;
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait AcceptBiStream: Streams {
    type Error: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn accept_bi(&self) -> Result<BiStreamsFor<Self>, Self::Error>;
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait OpeningUniStream: maybe::Send {
    type Streams: Streams;
    type Error: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn wait_uni(self) -> Result<<Self::Streams as Streams>::SendStream, Self::Error>;
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait OpenUniStream: Streams {
    type Opening: OpeningUniStream;
    type Error: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn open_uni(&self) -> Result<Self::Opening, Self::Error>;
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait AcceptUniStream: Streams {
    type Error: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn accept_uni(&self) -> Result<Self::RecvStream, Self::Error>;
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait Connecting: maybe::Send {
    type Connection: maybe::Send;
    type Error: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn wait(self) -> Result<Self::Connection, Self::Error>;
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait EndpointConnect: Sized + maybe::Send {
    type Params<'params>;
    type Connecting: Connecting;
    type Error: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn connect(
        &self,
        url: &str,
        params: Self::Params<'_>,
    ) -> Result<Self::Connecting, Self::Error>;
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait EndpointAccept: Sized + maybe::Send {
    type Connecting: Connecting;
    type Error: std::error::Error + maybe::Send + maybe::Sync + 'static;

    async fn accept(&self) -> Result<Self::Connecting, Self::Error>;
}

pub trait Connection:
    Streams + OpenBiStream + OpenUniStream + AcceptBiStream + AcceptUniStream
{
}

impl<T> Connection for T where
    T: Streams + OpenBiStream + OpenUniStream + AcceptBiStream + AcceptUniStream
{
}

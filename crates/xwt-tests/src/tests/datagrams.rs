use xwt_core::prelude::*;

const MIN_DATAGRAM_SIZE: usize = 1024;

#[derive(Debug, thiserror::Error)]
pub enum Error<Endpoint>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>:
        xwt_core::session::datagram::Send + xwt_core::session::datagram::Receive + std::fmt::Debug,
    ReceiveDatagramFor<ConnectSessionFor<Endpoint>>: std::fmt::Debug,
{
    #[error("connect: {0}")]
    Connect(#[source] xwt_error::Connect<Endpoint>),
    #[error("datagrams not supported")]
    DatagramsNotSupported,
    #[error("max datagram size is smaller than minimum - {size} / {MIN_DATAGRAM_SIZE}")]
    MaxDatagramSizeTooSmall { size: usize },
    #[error("send: {0}")]
    Send(#[source] SendErrorFor<ConnectSessionFor<Endpoint>>),
    #[error("recv: {0}")]
    Recv(#[source] ReceiveErrorFor<ConnectSessionFor<Endpoint>>),
    #[error("bad data")]
    BadData(ReceiveDatagramFor<ConnectSessionFor<Endpoint>>),
}

pub async fn run<Endpoint>(endpoint: Endpoint, url: &str) -> Result<(), Error<Endpoint>>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>: xwt_core::session::datagram::MaxSize
        + xwt_core::session::datagram::Send
        + xwt_core::session::datagram::Receive
        + std::fmt::Debug,
    ReceiveDatagramFor<ConnectSessionFor<Endpoint>>: std::fmt::Debug,
{
    let session = crate::utils::connect(&endpoint, url)
        .await
        .map_err(Error::Connect)?;

    let max_datagram_size = session
        .max_datagram_size()
        .ok_or(Error::DatagramsNotSupported)?;

    if max_datagram_size < MIN_DATAGRAM_SIZE {
        return Err(Error::MaxDatagramSizeTooSmall {
            size: max_datagram_size,
        });
    }

    session
        .send_datagram(&b"hello"[..])
        .await
        .map_err(Error::Send)?;

    let read = session.receive_datagram().await.map_err(Error::Recv)?;

    if read.as_ref() != b"hello" {
        return Err(Error::BadData(read));
    }

    Ok(())
}

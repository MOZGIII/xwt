use xwt_core::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum Error<Endpoint>
where
    Endpoint: xwt_core::EndpointConnect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    EndpointConnectConnectionFor<Endpoint>: xwt_core::datagram::Datagrams + std::fmt::Debug,
    ReceiveDatagramFor<EndpointConnectConnectionFor<Endpoint>>: std::fmt::Debug,
{
    #[error("connect: {0}")]
    Connect(#[source] xwt_error::Connect<Endpoint>),
    #[error("send: {0}")]
    Send(#[source] SendErrorFor<EndpointConnectConnectionFor<Endpoint>>),
    #[error("recv: {0}")]
    Recv(#[source] ReceiveErrorFor<EndpointConnectConnectionFor<Endpoint>>),
    #[error("bad data")]
    BadData(ReceiveDatagramFor<EndpointConnectConnectionFor<Endpoint>>),
}

pub async fn run<Endpoint>(endpoint: Endpoint, url: &str) -> Result<(), Error<Endpoint>>
where
    Endpoint: xwt_core::EndpointConnect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    EndpointConnectConnectionFor<Endpoint>: xwt_core::datagram::Datagrams + std::fmt::Debug,
    ReceiveDatagramFor<EndpointConnectConnectionFor<Endpoint>>: std::fmt::Debug,
{
    let connection = crate::utils::connect(&endpoint, url)
        .await
        .map_err(Error::Connect)?;

    connection
        .send_datagram(&b"hello"[..])
        .await
        .map_err(Error::Send)?;

    let read = connection.receive_datagram().await.map_err(Error::Recv)?;

    if read.as_ref() != b"hello" {
        return Err(Error::BadData(read));
    }

    Ok(())
}

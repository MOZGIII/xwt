use xwt_core::{datagram::ReceiveInto, prelude::*};

#[derive(Debug, thiserror::Error)]
pub enum Error<Endpoint>
where
    Endpoint: xwt_core::EndpointConnect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    EndpointConnectConnectionFor<Endpoint>:
        xwt_core::datagram::Send + xwt_core::datagram::ReceiveInto + std::fmt::Debug,
{
    #[error("connect: {0}")]
    Connect(#[source] xwt_error::Connect<Endpoint>),
    #[error("send: {0}")]
    Send(#[source] SendErrorFor<EndpointConnectConnectionFor<Endpoint>>),
    #[error("recv: {0}")]
    Recv(#[source] ReceiveIntoErrorFor<EndpointConnectConnectionFor<Endpoint>>),
    #[error("bad data")]
    BadData(Vec<u8>),
}

pub async fn run<Endpoint>(endpoint: Endpoint, url: &str) -> Result<(), Error<Endpoint>>
where
    Endpoint: xwt_core::EndpointConnect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    EndpointConnectConnectionFor<Endpoint>:
        xwt_core::datagram::Send + xwt_core::datagram::ReceiveInto + std::fmt::Debug,
{
    let connection = crate::utils::connect(&endpoint, url)
        .await
        .map_err(Error::Connect)?;

    connection
        .send_datagram(&b"hello"[..])
        .await
        .map_err(Error::Send)?;

    let mut read_buffer = [0; 10];

    let read_len = connection
        .receive_datagram_into(&mut read_buffer)
        .await
        .map_err(Error::Recv)?;

    let read = &read_buffer[..read_len];

    if read != b"hello" {
        return Err(Error::BadData(read.to_vec()));
    }

    Ok(())
}

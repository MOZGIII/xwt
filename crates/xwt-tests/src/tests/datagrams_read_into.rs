use xwt_core::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum Error<Endpoint>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>: xwt_core::session::datagram::Send
        + xwt_core::session::datagram::ReceiveInto
        + std::fmt::Debug,
{
    #[error("connect: {0}")]
    Connect(#[source] xwt_error::Connect<Endpoint>),
    #[error("send: {0}")]
    Send(#[source] SendErrorFor<ConnectSessionFor<Endpoint>>),
    #[error("recv: {0}")]
    Recv(#[source] ReceiveIntoErrorFor<ConnectSessionFor<Endpoint>>),
    #[error("bad data")]
    BadData(Vec<u8>),
}

pub async fn run<Endpoint>(endpoint: Endpoint, url: &str) -> Result<(), Error<Endpoint>>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>: xwt_core::session::datagram::Send
        + xwt_core::session::datagram::ReceiveInto
        + std::fmt::Debug,
{
    let session = crate::utils::connect(&endpoint, url)
        .await
        .map_err(Error::Connect)?;

    session
        .send_datagram(&b"hello"[..])
        .await
        .map_err(Error::Send)?;

    let mut read_buffer = [0; 10];

    let read_len = session
        .receive_datagram_into(&mut read_buffer)
        .await
        .map_err(Error::Recv)?;

    let read = &read_buffer[..read_len];

    if read != b"hello" {
        return Err(Error::BadData(read.to_vec()));
    }

    Ok(())
}

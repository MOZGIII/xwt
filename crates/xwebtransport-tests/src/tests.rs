use tokio::io::{AsyncReadExt, AsyncWriteExt};
use xwebtransport_core::trait_utils::EndpointConnectConnectionFor;

#[derive(Debug, thiserror::Error)]
pub enum EchoError<Endpoint>
where
    Endpoint: xwebtransport_core::EndpointConnect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    EndpointConnectConnectionFor<Endpoint>: xwebtransport_core::OpenBiStream + std::fmt::Debug,
{
    Connect(xwebtransport_error::Connect<Endpoint>),
    Open(xwebtransport_error::OpenBi<EndpointConnectConnectionFor<Endpoint>>),
    Send(std::io::Error),
    Recv(std::io::Error),
    BadData(Vec<u8>),
}

pub async fn echo<Endpoint>(endpoint: Endpoint) -> Result<(), EchoError<Endpoint>>
where
    Endpoint: xwebtransport_core::EndpointConnect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    EndpointConnectConnectionFor<Endpoint>: xwebtransport_core::OpenBiStream + std::fmt::Debug,
{
    let connection = crate::utils::connect(endpoint, "https://echo.webtransport.day")
        .await
        .map_err(EchoError::Connect)?;

    let (send_stream, recv_stream) = crate::utils::open_bi(connection)
        .await
        .map_err(EchoError::Open)?;

    let mut send_stream = std::pin::pin!(send_stream);

    send_stream
        .write_all(b"hello")
        .await
        .map_err(EchoError::Send)?;

    let mut recv_stream = std::pin::pin!(recv_stream);

    let mut data = Vec::with_capacity(1024);
    let read = recv_stream
        .read_buf(&mut data)
        .await
        .map_err(EchoError::Recv)?;

    if &data[..read] != b"hello" {
        return Err(EchoError::BadData(data));
    }

    Ok(())
}

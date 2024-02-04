use xwt_core::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum Error<Endpoint>
where
    Endpoint: xwt_core::EndpointConnect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    EndpointConnectConnectionFor<Endpoint>: xwt_core::OpenBiStream + std::fmt::Debug,
{
    #[error("connect: {0}")]
    Connect(#[source] xwt_error::Connect<Endpoint>),
    #[error("open: {0}")]
    Open(#[source] xwt_error::OpenBi<EndpointConnectConnectionFor<Endpoint>>),
    #[error("write: {0}")]
    Write(#[source] std::io::Error),
    #[error("read: {0}")]
    Read(#[source] std::io::Error),
    #[error("no response")]
    NoResponse,
    #[error("bad data")]
    BadData(Vec<u8>),
}

pub async fn run<Endpoint>(endpoint: Endpoint, url: &str) -> Result<(), Error<Endpoint>>
where
    Endpoint: xwt_core::EndpointConnect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    EndpointConnectConnectionFor<Endpoint>: xwt_core::OpenBiStream + std::fmt::Debug,
    SendStreamFor<EndpointConnectConnectionFor<Endpoint>>: tokio::io::AsyncWrite,
    RecvStreamFor<EndpointConnectConnectionFor<Endpoint>>: tokio::io::AsyncRead,
{
    let connection = crate::utils::connect(&endpoint, url)
        .await
        .map_err(Error::Connect)?;

    let (send_stream, recv_stream) = crate::utils::open_bi(&connection)
        .await
        .map_err(Error::Open)?;

    tokio::pin!(send_stream);

    let mut to_write = &b"hello"[..];
    loop {
        let written = tokio::io::AsyncWriteExt::write(&mut send_stream, to_write)
            .await
            .map_err(Error::Write)?;
        to_write = &to_write[written..];
        if to_write.is_empty() {
            break;
        }
    }

    tokio::pin!(recv_stream);

    let mut read_buf = vec![0u8; 1024];

    let read = tokio::io::AsyncReadExt::read(&mut recv_stream, &mut read_buf[..])
        .await
        .map_err(Error::Read)?;
    if read == 0 {
        return Err(Error::NoResponse);
    };
    read_buf.truncate(read);

    if read_buf != b"hello" {
        return Err(Error::BadData(read_buf));
    }

    Ok(())
}

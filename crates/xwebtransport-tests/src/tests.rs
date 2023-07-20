use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, thiserror::Error)]
pub enum EchoError<Connection>
where
    Connection: xwebtransport_core::OpenBiStream,
{
    Open(xwebtransport_error::OpenBi<Connection>),
    Send(std::io::Error),
    Recv(std::io::Error),
    BadData(Vec<u8>),
}

pub async fn echo<Connection>(connection: Connection) -> Result<(), EchoError<Connection>>
where
    Connection: xwebtransport_core::Connection,
{
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

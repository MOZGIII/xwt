//! The implementation of the echo server.

use crate::handling::{AcceptSessionRequestWith, RouteSession};

pub struct Route;

impl RouteSession for Route {
    const PATH: &'static str = "/echo";

    fn handler() -> impl crate::handling::HandleSessionRequest {
        AcceptSessionRequestWith((serve_streams, serve_datagrams))
    }
}

pub async fn serve_streams(
    connection: impl AsRef<wtransport::Connection>,
) -> Result<(), wtransport::error::ConnectionError> {
    let connection = connection.as_ref();
    loop {
        let stream = connection.accept_bi().await?;
        tokio::spawn(async move {
            tracing::info!(message = "serving stream");
            if let Err(error) = serve_stream(stream).await {
                tracing::error!(message = "error while serving stream", %error);
            }
            tracing::info!(message = "done serving stream");
        });
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StreamError {
    #[error("read: {0}")]
    Read(wtransport::error::StreamReadError),
    #[error("write: {0}")]
    Write(wtransport::error::StreamWriteError),
}

pub async fn serve_stream(
    stream: (wtransport::SendStream, wtransport::RecvStream),
) -> Result<(), StreamError> {
    let (mut tx, mut rx) = stream;
    let mut buf = vec![0; 1024];
    loop {
        let Some(len) = rx.read(&mut buf).await.map_err(StreamError::Read)? else {
            tracing::info!(message = "stream closed");
            return Ok(());
        };
        tracing::info!(message = "read stream data", %len);
        tx.write_all(&buf[..len])
            .await
            .map_err(StreamError::Write)?;
        tracing::info!(message = "written stream data", %len);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DatagramError {
    #[error("receive: {0}")]
    Receive(wtransport::error::ConnectionError),
    #[error("send: {0}")]
    Send(wtransport::error::SendDatagramError),
}

pub async fn serve_datagrams(
    connection: impl AsRef<wtransport::Connection>,
) -> Result<(), DatagramError> {
    let connection = connection.as_ref();
    loop {
        let datagram = connection
            .receive_datagram()
            .await
            .map_err(DatagramError::Receive)?;
        let size = datagram.payload().len();
        tracing::info!(message = "received datagram", %size);
        connection
            .send_datagram(datagram.payload())
            .map_err(DatagramError::Send)?;
        tracing::info!(message = "written datagram", %size);
    }
}

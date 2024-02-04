//! The implementation of the echo server.

use std::sync::Arc;

pub async fn serve_session_request(
    session_request: wtransport::endpoint::SessionRequest,
) -> Result<(), wtransport::error::ConnectionError> {
    tracing::info!(message = "accepting incoming session");
    let connection = session_request.accept().await?;

    tracing::info!(message = "new connection accepted");

    let connection = Arc::new(connection);

    let mut joinset = tokio::task::JoinSet::new();

    {
        let connection = Arc::clone(&connection);
        joinset.spawn(async move {
            tracing::info!(message = "serving streams");
            if let Err(error) = serve_streams(connection).await {
                tracing::error!(message = "error while serving streams", %error);
            }
            tracing::info!(message = "done serving streams");
        });
    }
    {
        let connection = Arc::clone(&connection);
        joinset.spawn(async move {
            tracing::info!(message = "serving datagrams");
            if let Err(error) = serve_datagrams(connection).await {
                tracing::error!(message = "error while serving datagrams", %error);
            }
            tracing::info!(message = "done serving datagrams");
        });
    }

    connection.closed().await;

    tracing::info!(message = "connection is closing");

    while let Some(result) = joinset.join_next().await {
        if let Err(panic) = result {
            tracing::error!(message = "panic in the connection task", %panic);
        }
    }

    tracing::info!(message = "connection tasks are finished");

    Ok(())
}

pub async fn serve_streams(
    connection: impl AsRef<wtransport::Connection>,
) -> Result<std::convert::Infallible, wtransport::error::ConnectionError> {
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

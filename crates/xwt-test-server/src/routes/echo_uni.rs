//! The implementation of the unidirectional stream echo server.

use crate::handling::{AcceptSessionRequestWith, RouteSession};

pub struct Route;

impl RouteSession for Route {
    const PATH: &'static str = "/echo-uni";

    fn handler() -> impl crate::handling::HandleSessionRequest {
        AcceptSessionRequestWith((serve_uni_streams,))
    }
}

pub async fn serve_uni_streams(
    connection: std::sync::Arc<wtransport::Connection>,
) -> Result<(), wtransport::error::ConnectionError> {
    loop {
        let recv_stream = connection.accept_uni().await?;
        let connection = std::sync::Arc::clone(&connection);
        tokio::spawn(async move {
            tracing::info!(message = "serving uni stream");
            if let Err(error) = serve_uni_stream(&connection, recv_stream).await {
                tracing::error!(message = "error while serving uni stream", %error);
            }
            tracing::info!(message = "done serving uni stream");
        });
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UniStreamError {
    #[error("read: {0}")]
    Read(wtransport::error::StreamReadError),
    #[error("open: {0}")]
    Open(wtransport::error::ConnectionError),
    #[error("opening: {0}")]
    Opening(wtransport::error::StreamOpeningError),
    #[error("write: {0}")]
    Write(wtransport::error::StreamWriteError),
}

/// Read the incoming unidirectional stream to the end and echo the data back
/// over a newly opened unidirectional stream.
///
/// The data is read in full before the response stream is opened, so the peer
/// has to finish the stream it sends to us to get a response.
pub async fn serve_uni_stream(
    connection: &wtransport::Connection,
    mut recv_stream: wtransport::RecvStream,
) -> Result<(), UniStreamError> {
    let mut data = Vec::new();
    let mut buf = vec![0; 1024];
    loop {
        let Some(len) = recv_stream
            .read(&mut buf)
            .await
            .map_err(UniStreamError::Read)?
        else {
            tracing::info!(message = "uni stream closed");
            break;
        };
        tracing::info!(message = "read uni stream data", %len);
        data.extend_from_slice(&buf[..len]);
    }

    let size = data.len();

    tracing::info!(message = "opening uni stream");
    let mut send_stream = connection
        .open_uni()
        .await
        .map_err(UniStreamError::Open)?
        .await
        .map_err(UniStreamError::Opening)?;

    send_stream
        .write_all(&data)
        .await
        .map_err(UniStreamError::Write)?;
    send_stream.finish().await.map_err(UniStreamError::Write)?;
    tracing::info!(message = "written uni stream data", %size);

    Ok(())
}

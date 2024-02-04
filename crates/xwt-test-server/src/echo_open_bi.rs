//! The handler that will open a bidi stream.

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
            tracing::info!(message = "opening bidi stream");
            if let Err(error) = open_bi_stream(connection).await {
                tracing::error!(message = "error while  opening bidi stream", %error);
            }
            tracing::info!(message = "done opening bidi stream");
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

pub async fn open_bi_stream(
    connection: impl AsRef<wtransport::Connection>,
) -> Result<(), wtransport::error::ConnectionError> {
    let connection = connection.as_ref();

    tracing::info!(message = "opening bi stream");
    let opening_bi_stream = connection.open_bi().await?;
    tokio::spawn(async move {
        tracing::info!(message = "serving opening bi stream");
        if let Err(error) = serve_opening_bi_stream(opening_bi_stream).await {
            tracing::error!(message = "error while serving stream", %error);
        }
        tracing::info!(message = "done serving opening bi stream");
    });

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum OpeningBiStreamError {
    #[error("opening: {0}")]
    Opening(wtransport::error::StreamOpeningError),
    #[error("serving: {0}")]
    Serving(crate::echo::StreamError),
}

pub async fn serve_opening_bi_stream(
    opening_bi_stream: wtransport::stream::OpeningBiStream,
) -> Result<(), OpeningBiStreamError> {
    let stream = opening_bi_stream
        .await
        .map_err(OpeningBiStreamError::Opening)?;
    crate::echo::serve_stream(stream)
        .await
        .map_err(OpeningBiStreamError::Serving)
}

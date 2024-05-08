//! The handler that will open a bidi stream.

use crate::handling::{AcceptSessionRequestWith, RouteSession};

pub struct Route;

impl RouteSession for Route {
    const PATH: &'static str = "/echo-open-bi";

    fn handler() -> impl crate::handling::HandleSessionRequest {
        AcceptSessionRequestWith((open_bi_stream,))
    }
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

//! The implementation of the server that observes the stream abortions that
//! the peer initiates and reports the observed error codes back.

use crate::handling::{AcceptSessionRequestWith, RouteSession};

pub struct AbortBiSend;

impl RouteSession for AbortBiSend {
    const PATH: &'static str = "/abort/bi/send";

    fn handler() -> impl crate::handling::HandleSessionRequest {
        AcceptSessionRequestWith((serve_abort_bi_send,))
    }
}

pub async fn serve_abort_bi_send(
    connection: std::sync::Arc<wtransport::Connection>,
) -> Result<(), wtransport::error::ConnectionError> {
    loop {
        let stream = connection.accept_bi().await?;
        let connection = std::sync::Arc::clone(&connection);
        tokio::spawn(async move {
            tracing::info!(message = "serving abort bi send stream");
            if let Err(error) = observe_and_report_reset(&connection, stream).await {
                tracing::error!(message = "error while serving stream", %error);
            }
            tracing::info!(message = "done serving abort bi send stream");
        });
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AbortBiSendError {
    #[error("read: {0}")]
    Read(wtransport::error::StreamReadError),
    #[error("write: {0}")]
    Write(wtransport::error::StreamWriteError),
    #[error("the stream has ended without being reset")]
    NotReset,
    #[error("unable to convert the HTTP error code {0} to a WebTransport error code")]
    BadErrorCode(u64),
    #[error("open: {0}")]
    Open(wtransport::error::ConnectionError),
    #[error("opening: {0}")]
    Opening(wtransport::error::StreamOpeningError),
}

/// Echo the first portion of the incoming data back, then wait for the peer to
/// reset the stream and report the error code it used over a newly opened
/// unidirectional stream.
///
/// The echo is what lets the peer know that we have this stream, so that it
/// only resets a stream that is already established at our side.
pub async fn observe_and_report_reset(
    connection: &wtransport::Connection,
    stream: (wtransport::SendStream, wtransport::RecvStream),
) -> Result<(), AbortBiSendError> {
    let (mut tx, mut rx) = stream;
    let mut buf = vec![0; 1024];

    let Some(len) = rx.read(&mut buf).await.map_err(AbortBiSendError::Read)? else {
        return Err(AbortBiSendError::NotReset);
    };
    tx.write_all(&buf[..len])
        .await
        .map_err(AbortBiSendError::Write)?;
    tracing::info!(message = "echoed the data back, awaiting the reset", %len);

    let error_code = loop {
        match rx.read(&mut buf).await {
            Ok(Some(len)) => {
                tracing::info!(message = "read more data while awaiting the reset", %len);
            }
            Ok(None) => return Err(AbortBiSendError::NotReset),
            Err(wtransport::error::StreamReadError::Reset(error_code)) => {
                break error_code.into_inner()
            }
            Err(error) => return Err(AbortBiSendError::Read(error)),
        }
    };

    let error_code = http_code_to_webtransport_code(error_code)
        .ok_or(AbortBiSendError::BadErrorCode(error_code))?;
    tracing::info!(message = "observed the stream reset", %error_code);

    let mut send_stream = connection
        .open_uni()
        .await
        .map_err(AbortBiSendError::Open)?
        .await
        .map_err(AbortBiSendError::Opening)?;
    send_stream
        .write_all(&error_code.to_be_bytes())
        .await
        .map_err(AbortBiSendError::Write)?;
    send_stream
        .finish()
        .await
        .map_err(AbortBiSendError::Write)?;

    Ok(())
}

/// Convert the HTTP3 error code into the WebTransport error code.
///
/// We need this because wtransport does not convert error codes at all.
fn http_code_to_webtransport_code(h: u64) -> Option<u32> {
    let first: u64 = 0x52e4a40fa8db;
    let last: u64 = 0x52e5ac983162;

    if h == 0 {
        return Some(0);
    }
    if !(first..=last).contains(&h) {
        return None;
    }

    let shifted = h - first;
    if shifted % 0x1f == 0x1e {
        return None;
    }
    (shifted - (shifted / 0x1f)).try_into().ok()
}

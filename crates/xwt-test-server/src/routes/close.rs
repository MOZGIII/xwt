//! The implementation of the close server.

use crate::handling::{AcceptSessionRequestWith, RouteSession};

pub struct CloseUni;

impl RouteSession for CloseUni {
    const PATH: &'static str = "/close/uni";

    fn handler() -> impl crate::handling::HandleSessionRequest {
        AcceptSessionRequestWith(accept_and_close_uni_streams)
    }
}

pub async fn accept_and_close_uni_streams(
    session: std::sync::Arc<wtransport::Connection>,
) -> Result<(), wtransport::error::ConnectionError> {
    let session = session.as_ref();
    loop {
        let stream = session.accept_uni().await?;
        stream.stop(0u8.into());
    }
}

pub struct CloseUniError;

impl RouteSession for CloseUniError {
    const PATH: &'static str = "/close/uni/error";

    fn handler() -> impl crate::handling::HandleSessionRequest {
        AcceptSessionRequestWith(accept_and_close_uni_streams_with_error_code)
    }
}

pub async fn accept_and_close_uni_streams_with_error_code(
    session: std::sync::Arc<wtransport::Connection>,
) -> Result<(), wtransport::error::ConnectionError> {
    let session = session.as_ref();
    loop {
        let stream = session.accept_uni().await?;
        stream.stop(webtransport_code_to_http_code(123));
    }
}

pub struct CloseBiRecv;

impl RouteSession for CloseBiRecv {
    const PATH: &'static str = "/close/bi/recv";

    fn handler() -> impl crate::handling::HandleSessionRequest {
        AcceptSessionRequestWith(accept_and_close_bi_recv_streams)
    }
}

pub async fn accept_and_close_bi_recv_streams(
    session: std::sync::Arc<wtransport::Connection>,
) -> Result<(), wtransport::error::ConnectionError> {
    let session = session.as_ref();
    loop {
        let (_send_stream, recv_stream) = session.accept_bi().await?;
        recv_stream.stop(0u8.into());
    }
}

pub struct CloseBiRecvError;

impl RouteSession for CloseBiRecvError {
    const PATH: &'static str = "/close/bi/recv/error";

    fn handler() -> impl crate::handling::HandleSessionRequest {
        AcceptSessionRequestWith(accept_and_close_bi_recv_streams_with_error_code)
    }
}

pub async fn accept_and_close_bi_recv_streams_with_error_code(
    session: std::sync::Arc<wtransport::Connection>,
) -> Result<(), wtransport::error::ConnectionError> {
    let session = session.as_ref();
    loop {
        let (_send_stream, recv_stream) = session.accept_bi().await?;
        recv_stream.stop(webtransport_code_to_http_code(123u8.into()));
    }
}

pub struct CloseBiSend;

impl RouteSession for CloseBiSend {
    const PATH: &'static str = "/close/bi/send";

    fn handler() -> impl crate::handling::HandleSessionRequest {
        AcceptSessionRequestWith(accept_and_close_bi_send_streams)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AcceptAndCloseBiSendStreamsError {
    #[error("connection: {0}")]
    Connection(wtransport::error::ConnectionError),
    #[error("stream write: {0}")]
    StreamWrite(wtransport::error::StreamWriteError),
}

pub async fn accept_and_close_bi_send_streams(
    session: std::sync::Arc<wtransport::Connection>,
) -> Result<(), AcceptAndCloseBiSendStreamsError> {
    let session = session.as_ref();
    loop {
        let (mut send_stream, _recv_stream) = session
            .accept_bi()
            .await
            .map_err(AcceptAndCloseBiSendStreamsError::Connection)?;
        send_stream
            .finish()
            .await
            .map_err(AcceptAndCloseBiSendStreamsError::StreamWrite)?;
    }
}

pub struct CloseBiSendError;

impl RouteSession for CloseBiSendError {
    const PATH: &'static str = "/close/bi/send/error";

    fn handler() -> impl crate::handling::HandleSessionRequest {
        AcceptSessionRequestWith(accept_and_close_bi_send_streams_with_error_code)
    }
}

pub async fn accept_and_close_bi_send_streams_with_error_code(
    session: std::sync::Arc<wtransport::Connection>,
) -> Result<(), wtransport::error::ConnectionError> {
    let session = session.as_ref();
    loop {
        let (mut send_stream, _recv_stream) = session.accept_bi().await?;
        if let Err(wtransport::error::ClosedStream) =
            send_stream.reset(webtransport_code_to_http_code(123))
        {
            tracing::debug!(message = "unable to reset a closed stream");
        }
    }
}

/// Convert the WebTransport error code into the HTTP3 error code.
///
/// We need this because wtransport does not convert error codes at all.
fn webtransport_code_to_http_code(n: u32) -> wtransport::VarInt {
    let first: u64 = 0x52e4a40fa8db;
    let n: u64 = n.into();
    let val: u64 = first + n + (n / 0x1e);
    val.try_into().unwrap()
}

//! This test ensures that aborting the write side of a stream propagates
//! the error code to the peer.
//!
//! The server echoes the data we send first - so that we only abort a stream
//! that is already established at the server side - and then reports the error
//! code it has observed over a unidirectional stream.

use xwt_core::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum Error<Endpoint>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>:
        xwt_core::session::stream::OpenBi + xwt_core::session::stream::AcceptUni + std::fmt::Debug,
{
    #[error("connect: {0}")]
    Connect(#[source] xwt_error::Connect<Endpoint>),
    #[error("open: {0}")]
    Open(#[source] xwt_error::OpenBi<ConnectSessionFor<Endpoint>>),
    #[error("send: {0}")]
    Send(#[source] WriteErrorFor<SendStreamFor<ConnectSessionFor<Endpoint>>>),
    #[error("recv: {0}")]
    Recv(#[source] ReadErrorFor<RecvStreamFor<ConnectSessionFor<Endpoint>>>),
    #[error("write stream abort: {0}")]
    WriteStreamAbort(#[source] WriteAbortErrorFor<SendStreamFor<ConnectSessionFor<Endpoint>>>),
    #[error("accept uni stream: {0}")]
    AcceptUniStream(#[source] UniStreamAcceptErrorFor<ConnectSessionFor<Endpoint>>),
    #[error("error code mismatch: got code {0}")]
    ErrorCodeMismatch(xwt_core::stream::ErrorCode),
}

pub async fn run<Endpoint>(
    endpoint: Endpoint,
    url: &str,
    error_code: xwt_core::stream::ErrorCode,
) -> Result<(), Error<Endpoint>>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>:
        xwt_core::session::stream::OpenBi + xwt_core::session::stream::AcceptUni + std::fmt::Debug,
{
    let session = crate::utils::connect(&endpoint, url)
        .await
        .map_err(Error::Connect)?;

    let (mut send_stream, mut recv_stream) =
        crate::utils::open_bi(&session).await.map_err(Error::Open)?;

    let mut to_write = &b"ping"[..];
    loop {
        let written = send_stream.write(to_write).await.map_err(Error::Send)?;
        let written = written.get();
        to_write = &to_write[written..];
        if to_write.is_empty() {
            break;
        }
    }

    // Wait for the echo to ensure the server has this stream before we abort
    // it - a stream that is reset before it reaches the peer might never be
    // observed there at all.
    let mut echo_buf = [0u8; 4];
    let mut filled = 0;
    while filled < echo_buf.len() {
        let read = recv_stream
            .read(&mut echo_buf[filled..])
            .await
            .map_err(Error::Recv)?;
        filled += read.get();
    }

    send_stream
        .abort(error_code)
        .await
        .map_err(Error::WriteStreamAbort)?;

    let mut report_stream = session.accept_uni().await.map_err(Error::AcceptUniStream)?;

    let mut report_buf = [0u8; 4];
    let mut filled = 0;
    while filled < report_buf.len() {
        let read = report_stream
            .read(&mut report_buf[filled..])
            .await
            .map_err(Error::Recv)?;
        filled += read.get();
    }

    let observed_error_code = xwt_core::stream::ErrorCode::from_be_bytes(report_buf);

    if observed_error_code != error_code {
        return Err(Error::ErrorCodeMismatch(observed_error_code));
    }

    Ok(())
}

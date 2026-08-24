//! This test exercises the unidirectional streams in both directions:
//! the client opens a unidirectional stream and finishes it, and then accepts
//! a unidirectional stream that the server opens to send the data back.

use xwt_core::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum Error<Endpoint>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>:
        xwt_core::session::stream::OpenUni + xwt_core::session::stream::AcceptUni + std::fmt::Debug,
{
    #[error("connect: {0}")]
    Connect(#[source] xwt_error::Connect<Endpoint>),
    #[error("open: {0}")]
    Open(#[source] xwt_error::OpenUni<ConnectSessionFor<Endpoint>>),
    #[error("send: {0}")]
    Send(#[source] WriteErrorFor<SendStreamFor<ConnectSessionFor<Endpoint>>>),
    #[error("finish: {0}")]
    Finish(#[source] FinishErrorFor<SendStreamFor<ConnectSessionFor<Endpoint>>>),
    #[error("accept uni stream: {0}")]
    AcceptUniStream(#[source] UniStreamAcceptErrorFor<ConnectSessionFor<Endpoint>>),
    #[error("recv: {0}")]
    Recv(#[source] ReadErrorFor<RecvStreamFor<ConnectSessionFor<Endpoint>>>),
    #[error("bad data")]
    BadData(Vec<u8>),
}

pub async fn run<Endpoint>(endpoint: Endpoint, url: &str) -> Result<(), Error<Endpoint>>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>:
        xwt_core::session::stream::OpenUni + xwt_core::session::stream::AcceptUni + std::fmt::Debug,
{
    let session = crate::utils::connect(&endpoint, url)
        .await
        .map_err(Error::Connect)?;

    let mut send_stream = crate::utils::open_uni(&session)
        .await
        .map_err(Error::Open)?;

    let mut to_write = &b"hello"[..];
    loop {
        let written = send_stream.write(to_write).await.map_err(Error::Send)?;
        let written = written.get();
        to_write = &to_write[written..];
        if to_write.is_empty() {
            break;
        }
    }

    // The server only responds once it has read our stream to the end, so
    // the stream has to be finished for the test to make progress.
    send_stream.finish().await.map_err(Error::Finish)?;

    let mut recv_stream = session.accept_uni().await.map_err(Error::AcceptUniStream)?;

    let mut read_buf = vec![0u8; 1024];

    let read = recv_stream
        .read(&mut read_buf[..])
        .await
        .map_err(Error::Recv)?;
    let read = read.get();
    read_buf.truncate(read);

    if read_buf != b"hello" {
        return Err(Error::BadData(read_buf));
    }

    Ok(())
}

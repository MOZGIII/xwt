//! This test ensures that when the session is dropped we get the expected
//! effect - which is that the streams, their readers and writers and
//! the datagrams readers and writers are closed.

use xwt_core::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum Error<Endpoint>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>: xwt_core::session::stream::OpenBi + std::fmt::Debug,
{
    #[error("connect: {0}")]
    Connect(#[source] xwt_error::Connect<Endpoint>),
    #[error("open: {0}")]
    Open(#[source] xwt_error::OpenBi<ConnectSessionFor<Endpoint>>),
    #[error("read has not failed")]
    ReadNotFailed,
    #[error("got a read error that was not expected: {0}")]
    UnexpectedReadError(ReadErrorFor<RecvStreamFor<ConnectSessionFor<Endpoint>>>),
}

pub async fn run<Endpoint>(
    endpoint: Endpoint,
    url: &str,
    checker: impl FnOnce(&ReadErrorFor<RecvStreamFor<ConnectSessionFor<Endpoint>>>) -> bool,
) -> Result<(), Error<Endpoint>>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>: xwt_core::session::stream::OpenBi + std::fmt::Debug,
{
    let session = crate::utils::connect(&endpoint, url)
        .await
        .map_err(Error::Connect)?;

    let (mut _send_stream, mut recv_stream) =
        crate::utils::open_bi(&session).await.map_err(Error::Open)?;

    drop(session);

    let mut buf = [0; 1024];
    let result = recv_stream.read(&mut buf).await;

    let Err(error) = result else {
        return Err(Error::ReadNotFailed);
    };

    if !(checker)(&error) {
        tracing::info!(message = "The error on the session drop was", ?error);
        return Err(Error::UnexpectedReadError(error));
    }

    Ok(())
}

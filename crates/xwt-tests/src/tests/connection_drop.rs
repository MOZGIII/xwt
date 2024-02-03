//! This test ensures that when the connection is dropped we get the expected
//! effect - which is that the streams, their readers and writers and
//! the datagrams readers and writers are closed.

use xwt_core::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum Error<Endpoint>
where
    Endpoint: xwt_core::EndpointConnect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    EndpointConnectConnectionFor<Endpoint>: xwt_core::OpenBiStream + std::fmt::Debug,
{
    #[error("connect: {0}")]
    Connect(#[source] xwt_error::Connect<Endpoint>),
    #[error("open: {0}")]
    Open(#[source] xwt_error::OpenBi<EndpointConnectConnectionFor<Endpoint>>),
    #[error("read has not failed")]
    ReadNotFailed,
    #[error("got a read error that was not expected: {0}")]
    UnexpectedReadError(ReadErrorFor<RecvStreamFor<EndpointConnectConnectionFor<Endpoint>>>),
}

pub async fn run<Endpoint>(
    endpoint: Endpoint,
    url: &str,
    checker: impl FnOnce(&ReadErrorFor<RecvStreamFor<EndpointConnectConnectionFor<Endpoint>>>) -> bool,
) -> Result<(), Error<Endpoint>>
where
    Endpoint: xwt_core::EndpointConnect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    EndpointConnectConnectionFor<Endpoint>: xwt_core::OpenBiStream + std::fmt::Debug,
{
    let connection = crate::utils::connect(&endpoint, url)
        .await
        .map_err(Error::Connect)?;

    let (mut _send_stream, mut recv_stream) = crate::utils::open_bi(&connection)
        .await
        .map_err(Error::Open)?;

    drop(connection);

    let mut buf = [0; 1024];
    let result = recv_stream.read(&mut buf).await;

    let Err(error) = result else {
        return Err(Error::ReadNotFailed);
    };

    if !(checker)(&error) {
        tracing::info!(message = "The error on the connection drop was", ?error);
        return Err(Error::UnexpectedReadError(error));
    }

    Ok(())
}

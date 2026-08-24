//! This test ensures that the read stream observes the clean finish of
//! the corresponding write side at the peer via
//! the [`xwt_core::stream::Finished`] API.

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
    #[error("read stream finished: {0}")]
    ReadStreamFinished(#[source] FinishedErrorFor<RecvStreamFor<ConnectSessionFor<Endpoint>>>),
}

pub async fn run<Endpoint>(endpoint: Endpoint, url: &str) -> Result<(), Error<Endpoint>>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>: xwt_core::session::stream::OpenBi + std::fmt::Debug,
{
    let session = crate::utils::connect(&endpoint, url)
        .await
        .map_err(Error::Connect)?;

    let (_send_stream, recv_stream) = crate::utils::open_bi(&session).await.map_err(Error::Open)?;

    recv_stream
        .finished()
        .await
        .map_err(Error::ReadStreamFinished)?;

    Ok(())
}

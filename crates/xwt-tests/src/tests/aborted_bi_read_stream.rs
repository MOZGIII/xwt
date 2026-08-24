//! This test ensures that the read stream observes the abortion of
//! the corresponding write side at the peer via
//! the [`xwt_core::stream::ReadAborted`] API.
//!
//! Only an actual abort is covered here: the native driver completes this
//! call solely on a `RESET_STREAM`, while the web driver also completes it on
//! a clean stream finish.

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
    #[error("read stream aborted: {0}")]
    ReadStreamAborted(#[source] ReadAbortedErrorFor<RecvStreamFor<ConnectSessionFor<Endpoint>>>),
    #[error("error code mismatch: got code {0}")]
    ErrorCodeMismatch(xwt_core::stream::ErrorCode),
}

pub async fn run<Endpoint>(
    endpoint: Endpoint,
    url: &str,
    expected_error_code: xwt_core::stream::ErrorCode,
) -> Result<(), Error<Endpoint>>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>: xwt_core::session::stream::OpenBi + std::fmt::Debug,
{
    let session = crate::utils::connect(&endpoint, url)
        .await
        .map_err(Error::Connect)?;

    let (_send_stream, recv_stream) = crate::utils::open_bi(&session).await.map_err(Error::Open)?;

    let error_code = recv_stream
        .aborted()
        .await
        .map_err(Error::ReadStreamAborted)?;

    if error_code != expected_error_code {
        return Err(Error::ErrorCodeMismatch(error_code));
    }

    Ok(())
}

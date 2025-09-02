use std::num::NonZeroUsize;

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
    #[error("open bi stream: {0}")]
    OpenBiStream(#[source] BiStreamOpenErrorFor<ConnectSessionFor<Endpoint>>),
    #[error("opening bi stream: {0}")]
    OpeningBiStream(#[source] BiStreamOpeningErrorFor<ConnectSessionFor<Endpoint>>),
    #[error("read stream: {0}")]
    ReadStream(#[source] ReadErrorFor<RecvStreamFor<ConnectSessionFor<Endpoint>>>),
    #[error("a read was successful while we expected it to abort (read {0} bytes)")]
    ReadDidNotFail(NonZeroUsize),
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
    RecvStreamFor<ConnectSessionFor<Endpoint>>: xwt_core::stream::Read,
{
    let session = crate::utils::connect(&endpoint, url)
        .await
        .map_err(Error::Connect)?;

    let opening = session.open_bi().await.map_err(Error::OpenBiStream)?;
    let (_send_stream, mut recv_stream) =
        opening.wait_bi().await.map_err(Error::OpeningBiStream)?;

    let mut buf = [0u8; 1];

    let error_code = match recv_stream.read(&mut buf).await {
        Ok(len) => return Err(Error::ReadDidNotFail(len)),
        Err(err) => match err.as_error_code() {
            Some(error_code) => error_code,
            None => return Err(Error::ReadStream(err)),
        },
    };

    if error_code != expected_error_code {
        return Err(Error::ErrorCodeMismatch(error_code));
    }

    Ok(())
}

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
    #[error("write stream aborted: {0}")]
    WriteStreamAborted(#[source] WriteAbortedErrorFor<SendStreamFor<ConnectSessionFor<Endpoint>>>),
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
    SendStreamFor<ConnectSessionFor<Endpoint>>: xwt_core::stream::WriteAborted,
{
    let session = crate::utils::connect(&endpoint, url)
        .await
        .map_err(Error::Connect)?;

    let opening = session.open_bi().await.map_err(Error::OpenBiStream)?;
    let (send_stream, _recv_stream) = opening.wait_bi().await.map_err(Error::OpeningBiStream)?;

    let error_code = send_stream
        .aborted()
        .await
        .map_err(Error::WriteStreamAborted)?;

    if error_code != expected_error_code {
        return Err(Error::ErrorCodeMismatch(error_code));
    }

    Ok(())
}

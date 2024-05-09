use xwt_core::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum Error<Endpoint>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>: xwt_core::session::stream::OpenUni + std::fmt::Debug,
{
    #[error("connect: {0}")]
    Connect(#[source] xwt_error::Connect<Endpoint>),
    #[error("open uni stream: {0}")]
    OpenUniStream(#[source] UniStreamOpenErrorFor<ConnectSessionFor<Endpoint>>),
    #[error("opening uni stream: {0}")]
    OpeningUniStream(#[source] UniStreamOpeningErrorFor<ConnectSessionFor<Endpoint>>),
    #[error("stream aborted: {0}")]
    StreamAborted(#[source] WriteAbortedErrorFor<SendStreamFor<ConnectSessionFor<Endpoint>>>),
    #[error("error code conversion to u8 failed")]
    ErrorCodeConversion,
    #[error("error code mismatch: got code {0}")]
    ErrorCodeMismatch(u8),
}

pub async fn run<Endpoint>(
    endpoint: Endpoint,
    url: &str,
    expected_error_code: u8,
) -> Result<(), Error<Endpoint>>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
    ConnectSessionFor<Endpoint>: xwt_core::session::stream::OpenUni + std::fmt::Debug,
    SendStreamFor<ConnectSessionFor<Endpoint>>: xwt_core::stream::WriteAborted,
{
    let session = crate::utils::connect(&endpoint, url)
        .await
        .map_err(Error::Connect)?;

    let opening = session.open_uni().await.map_err(Error::OpenUniStream)?;
    let send_stream = opening.wait_uni().await.map_err(Error::OpeningUniStream)?;

    let error_code = send_stream.aborted().await.map_err(Error::StreamAborted)?;

    let error_code: u8 = error_code
        .try_into()
        .map_err(|_| Error::ErrorCodeConversion)?;

    if error_code != expected_error_code {
        return Err(Error::ErrorCodeMismatch(error_code));
    }

    Ok(())
}
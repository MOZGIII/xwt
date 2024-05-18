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
    #[error("a read was successful while we expected it to abort")]
    ReadSuccessful,
    #[error("a read has successfully finished sending while we expected it to abort")]
    FinishSuccessful,
    #[error("error code conversion to u32 failed")]
    ErrorCodeConversion,
    #[error("error code mismatch: got code {0}")]
    ErrorCodeMismatch(u32),
}

pub async fn run<Endpoint>(
    endpoint: Endpoint,
    url: &str,
    expected_error_code: u32,
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
        Ok(Some(_len)) => return Err(Error::ReadSuccessful),
        Ok(None) => 0,
        Err(_err) => 1337,
    };

    let error_code: u32 = error_code
        .try_into()
        .map_err(|_| Error::ErrorCodeConversion)?;

    if error_code != expected_error_code {
        return Err(Error::ErrorCodeMismatch(error_code));
    }

    Ok(())
}

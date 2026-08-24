//! This test ensures that a session that the server refuses to accept is
//! reported as a connection failure rather than as a connected session.

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("connect has not failed")]
    ConnectDidNotFail,
}

pub async fn run<Endpoint>(endpoint: Endpoint, url: &str) -> Result<(), Error>
where
    Endpoint: xwt_core::endpoint::Connect + std::fmt::Debug,
    Endpoint::Connecting: std::fmt::Debug,
{
    let result = crate::utils::connect(&endpoint, url).await;

    let Err(error) = result else {
        return Err(Error::ConnectDidNotFail);
    };

    tracing::info!(message = "the error on the rejected connect was", %error);

    Ok(())
}

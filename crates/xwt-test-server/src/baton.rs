//! The handler that implements [draft-frindell-webtrans-devious-baton-00][1].
//!
//! [1]: https://datatracker.ietf.org/doc/html/draft-frindell-webtrans-devious-baton

pub async fn serve_session_request(
    session_request: wtransport::endpoint::SessionRequest,
) -> Result<(), wtransport::error::ConnectionError> {
    tracing::info!(message = "accepting incoming session");
    let connection = session_request.accept().await?;

    todo!();

    Ok(())
}

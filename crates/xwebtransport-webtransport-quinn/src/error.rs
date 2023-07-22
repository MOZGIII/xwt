#[derive(Debug, thiserror::Error)]
pub enum ConnectError {
    #[error("uri: {0}")]
    Uri(http::uri::InvalidUri),
    #[error("client: {0}")]
    Client(webtransport_quinn::ClientError),
}

#[derive(Debug, thiserror::Error)]
pub enum AcceptingError {
    #[error("connection: {0}")]
    Connection(quinn::ConnectionError),
    #[error("server: {0}")]
    Server(webtransport_quinn::ServerError),
}

#![cfg(not(target_family = "wasm"))]

use xwebtransport_core::async_trait;

mod error;
mod types;

pub use self::error::*;
pub use self::types::*;

#[async_trait]
impl xwebtransport_core::traits::EndpointConnect for Endpoint {
    type Connecting = xwebtransport_core::utils::dummy::Connecting<Session>;
    type Error = ConnectError;

    async fn connect(&self, url: &str) -> Result<Self::Connecting, Self::Error> {
        let url = http::uri::Uri::try_from(url).map_err(ConnectError::Uri)?;
        let session = webtransport_quinn::connect(&self.0, &url)
            .await
            .map(Session)
            .map_err(ConnectError::Client)?;
        Ok(xwebtransport_core::utils::dummy::Connecting(session))
    }
}

#[async_trait]
impl xwebtransport_core::traits::EndpointAccept for Endpoint {
    type Accepting = Connecting;
    type Error = std::convert::Infallible;

    async fn accept(&self) -> Result<Option<Self::Accepting>, Self::Error> {
        let maybe_connecting = self.0.accept().await;
        Ok(maybe_connecting.map(Connecting))
    }
}

#[async_trait]
impl xwebtransport_core::traits::Accepting for Connecting {
    type Request = Request;
    type Error = AcceptingError;

    async fn wait_accept(self) -> Result<Self::Request, Self::Error> {
        let connection = self.0.await.map_err(AcceptingError::Connection)?;
        webtransport_quinn::accept(connection)
            .await
            .map(Request)
            .map_err(AcceptingError::Server)
    }
}

#[async_trait]
impl xwebtransport_core::traits::Request for Request {
    type Connection = Session;
    type OkError = quinn::WriteError;
    type CloseError = quinn::WriteError;

    async fn ok(self) -> Result<Self::Connection, Self::OkError> {
        self.0.ok().await.map(Session)
    }

    async fn close(self, status: u16) -> Result<(), Self::CloseError> {
        let status_code = http::StatusCode::from_u16(status).unwrap();
        self.0.close(status_code).await
    }
}

impl xwebtransport_core::traits::Streams for Session {
    type SendStream = webtransport_quinn::SendStream;
    type RecvStream = webtransport_quinn::RecvStream;
}

#[async_trait]
impl xwebtransport_core::traits::OpenBiStream for Session {
    type Opening = xwebtransport_core::utils::dummy::OpeningBiStream<Self>;
    type Error = webtransport_quinn::SessionError;

    async fn open_bi(&self) -> Result<Self::Opening, Self::Error> {
        self.0
            .open_bi()
            .await
            .map(xwebtransport_core::utils::dummy::OpeningBiStream)
    }
}

#[async_trait]
impl xwebtransport_core::traits::AcceptBiStream for Session {
    type Error = webtransport_quinn::SessionError;

    async fn accept_bi(
        &self,
    ) -> Result<xwebtransport_core::traits::BiStreamsFor<Self>, Self::Error> {
        self.0.accept_bi().await
    }
}

#[async_trait]
impl xwebtransport_core::traits::OpenUniStream for Session {
    type Opening = xwebtransport_core::utils::dummy::OpeningUniStream<Self>;
    type Error = webtransport_quinn::SessionError;

    async fn open_uni(&self) -> Result<Self::Opening, Self::Error> {
        self.0
            .open_uni()
            .await
            .map(xwebtransport_core::utils::dummy::OpeningUniStream)
    }
}

#[async_trait]
impl xwebtransport_core::traits::AcceptUniStream for Session {
    type Error = webtransport_quinn::SessionError;

    async fn accept_uni(&self) -> Result<Self::RecvStream, Self::Error> {
        self.0.accept_uni().await
    }
}

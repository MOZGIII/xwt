#![cfg(not(target_family = "wasm"))]

use xwebtransport_core::async_trait;

mod types;

pub use self::types::*;

#[async_trait]
impl xwebtransport_core::traits::EndpointConnect for Endpoint<wtransport::endpoint::Client> {
    type Params = ();
    type Connecting = xwebtransport_core::utils::dummy::Connecting<Connection>;
    type Error = wtransport::error::ConnectingError;

    async fn connect(
        &self,
        url: &str,
        _params: &Self::Params,
    ) -> Result<Self::Connecting, Self::Error> {
        let connecting = self.0.connect(url).await.map(Connection)?;
        Ok(xwebtransport_core::utils::dummy::Connecting(connecting))
    }
}

#[async_trait]
impl xwebtransport_core::traits::EndpointAccept for Endpoint<wtransport::endpoint::Server> {
    type Accepting = IncomingSession;
    type Error = std::convert::Infallible;

    async fn accept(&self) -> Result<Option<Self::Accepting>, Self::Error> {
        let incoming_session = self.0.accept().await;
        let incoming_session = IncomingSession(incoming_session);
        Ok(Some(incoming_session))
    }
}

#[async_trait]
impl xwebtransport_core::traits::Accepting for IncomingSession {
    type Request = SessionRequest;
    type Error = wtransport::error::ConnectionError;

    async fn wait_accept(self) -> Result<Self::Request, Self::Error> {
        self.0.await.map(SessionRequest)
    }
}

#[async_trait]
impl xwebtransport_core::traits::Request for SessionRequest {
    type Connection = Connection;
    type OkError = wtransport::error::ConnectionError;
    type CloseError = std::convert::Infallible;

    async fn ok(self) -> Result<Self::Connection, Self::OkError> {
        self.0.accept().await.map(Connection)
    }

    async fn close(self, status: u16) -> Result<(), Self::CloseError> {
        debug_assert!(
            status == 404,
            "wtransport driver only supports closing requests with 404 status code"
        );
        self.0.not_found().await;
        Ok(())
    }
}

impl xwebtransport_core::traits::Streams for Connection {
    type SendStream = wtransport::SendStream;
    type RecvStream = wtransport::RecvStream;
}

#[async_trait]
impl xwebtransport_core::traits::OpeningBiStream for OpeningBiStream {
    type Streams = Connection;
    type Error = wtransport::error::StreamOpeningError;

    async fn wait_bi(
        self,
    ) -> Result<xwebtransport_core::traits::BiStreamsFor<Self::Streams>, Self::Error> {
        self.0.await
    }
}

#[async_trait]
impl xwebtransport_core::traits::OpenBiStream for Connection {
    type Opening = OpeningBiStream;
    type Error = wtransport::error::ConnectionError;

    async fn open_bi(&self) -> Result<Self::Opening, Self::Error> {
        self.0.open_bi().await.map(OpeningBiStream)
    }
}

#[async_trait]
impl xwebtransport_core::traits::AcceptBiStream for Connection {
    type Error = wtransport::error::ConnectionError;

    async fn accept_bi(
        &self,
    ) -> Result<xwebtransport_core::traits::BiStreamsFor<Self>, Self::Error> {
        self.0.accept_bi().await
    }
}

#[async_trait]
impl xwebtransport_core::traits::OpeningUniStream for OpeningUniStream {
    type Streams = Connection;
    type Error = wtransport::error::StreamOpeningError;

    async fn wait_uni(
        self,
    ) -> Result<<Self::Streams as xwebtransport_core::traits::Streams>::SendStream, Self::Error>
    {
        self.0.await
    }
}

#[async_trait]
impl xwebtransport_core::traits::OpenUniStream for Connection {
    type Opening = OpeningUniStream;
    type Error = wtransport::error::ConnectionError;

    async fn open_uni(&self) -> Result<Self::Opening, Self::Error> {
        self.0.open_uni().await.map(OpeningUniStream)
    }
}

#[async_trait]
impl xwebtransport_core::traits::AcceptUniStream for Connection {
    type Error = wtransport::error::ConnectionError;

    async fn accept_uni(&self) -> Result<Self::RecvStream, Self::Error> {
        self.0.accept_uni().await
    }
}

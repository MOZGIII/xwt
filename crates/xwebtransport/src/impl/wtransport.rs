impl crate::traits::EndpointConnect for wtransport::Endpoint<wtransport::endpoint::Client> {
    type ConnectError = wtransport::error::ConnectingError;
    type Params<'params> = ();
    type Connecting = crate::utils::dummy::Connecting<wtransport::Connection>;

    async fn connect(
        &self,
        url: &str,
        _params: Self::Params<'_>,
    ) -> Result<Self::Connecting, Self::ConnectError> {
        Ok(crate::utils::dummy::Connecting(self.connect(url).await?))
    }
}

impl crate::traits::EndpointAccept for wtransport::Endpoint<wtransport::endpoint::Server> {
    type AcceptError = wtransport::error::ConnectionError;
    type Connecting = wtransport::endpoint::SessionRequest;

    async fn accept(&self) -> Result<Self::Connecting, Self::AcceptError> {
        let incoming = self.accept().await;
        incoming.await
    }
}

impl crate::traits::Connecting for wtransport::endpoint::SessionRequest {
    type Error = wtransport::error::ConnectionError;
    type Connection = wtransport::Connection;

    async fn wait(self) -> Result<Self::Connection, Self::Error> {
        self.accept().await
    }
}

impl crate::traits::OpeningBiStream for wtransport::stream::OpeningBiStream {
    type SendStream = wtransport::SendStream;
    type RecvStream = wtransport::RecvStream;
    type Error = wtransport::error::StreamOpeningError;

    async fn wait(self) -> Result<(Self::SendStream, Self::RecvStream), Self::Error> {
        self.await
    }
}

impl crate::traits::OpenBiStream for wtransport::Connection {
    type Opening = wtransport::stream::OpeningBiStream;
    type Error = wtransport::error::ConnectionError;

    async fn open_bi(&self) -> Result<Self::Opening, Self::Error> {
        self.open_bi().await
    }
}

impl crate::traits::AcceptBiStream for wtransport::Connection {
    type SendStream = wtransport::SendStream;
    type RecvStream = wtransport::RecvStream;
    type Error = wtransport::error::ConnectionError;

    async fn accept_bi(&self) -> Result<(Self::SendStream, Self::RecvStream), Self::Error> {
        self.accept_bi().await
    }
}

impl crate::traits::OpeningUniStream for wtransport::stream::OpeningUniStream {
    type SendStream = wtransport::SendStream;
    type Error = wtransport::error::StreamOpeningError;

    async fn wait(self) -> Result<Self::SendStream, Self::Error> {
        self.await
    }
}

impl crate::traits::OpenUniStream for wtransport::Connection {
    type Opening = wtransport::stream::OpeningUniStream;
    type Error = wtransport::error::ConnectionError;

    async fn open_uni(&self) -> Result<Self::Opening, Self::Error> {
        self.open_uni().await
    }
}

impl crate::traits::AcceptUniStream for wtransport::Connection {
    type RecvStream = wtransport::RecvStream;
    type Error = wtransport::error::ConnectionError;

    async fn accept_uni(&self) -> Result<Self::RecvStream, Self::Error> {
        self.accept_uni().await
    }
}

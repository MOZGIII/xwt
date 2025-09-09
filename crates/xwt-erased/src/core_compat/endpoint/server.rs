impl xwt_core::endpoint::Accept for crate::endpoint::Server {
    type Accepting = crate::endpoint::server::Accepting;
    type Error = crate::Error;

    async fn accept(&self) -> Result<Option<Self::Accepting>, Self::Error> {
        self.accept().await
    }
}

impl xwt_core::endpoint::accept::Accepting for crate::endpoint::server::Accepting {
    type Request = crate::endpoint::server::Request;
    type Error = crate::Error;

    async fn wait_accept(self) -> Result<Self::Request, Self::Error> {
        self.wait_accept().await
    }
}

impl xwt_core::endpoint::accept::Request for crate::endpoint::server::Request {
    type Session = crate::Session;
    type OkError = crate::Error;
    type CloseError = crate::Error;

    async fn ok(self) -> Result<Self::Session, Self::OkError> {
        self.ok().await
    }

    async fn close(self, status: u16) -> Result<(), Self::CloseError> {
        self.close(status).await
    }
}

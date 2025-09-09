impl xwt_core::endpoint::Connect for crate::endpoint::Client {
    type Connecting = crate::endpoint::client::Connecting;
    type Error = crate::Error;

    async fn connect(&self, url: &str) -> Result<Self::Connecting, Self::Error> {
        self.connect(url).await
    }
}

impl xwt_core::endpoint::connect::Connecting for crate::endpoint::client::Connecting {
    type Session = crate::Session;
    type Error = crate::Error;

    async fn wait_connect(self) -> Result<Self::Session, Self::Error> {
        self.wait_connect().await
    }
}

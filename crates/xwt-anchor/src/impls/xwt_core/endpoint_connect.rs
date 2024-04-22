//! Implementations related to endpoint connect operations.

use xwt_core::utils::maybe;

use crate::types::*;

impl<T> xwt_core::EndpointConnect for Endpoint<T>
where
    T: xwt_core::EndpointConnect + maybe::Send + maybe::Sync,
{
    type Connecting = Connecting<T::Connecting>;
    type Error = T::Error;

    async fn connect(&self, url: &str) -> Result<Self::Connecting, Self::Error> {
        T::connect(&self.0, url).await.map(Connecting)
    }
}

impl<T> xwt_core::Connecting for Connecting<T>
where
    T: xwt_core::Connecting,
{
    type Connection = Connection<T::Connection>;
    type Error = T::Error;

    async fn wait_connect(self) -> Result<Self::Connection, Self::Error> {
        T::wait_connect(self.0).await.map(Connection)
    }
}

//! Implementations related to session datagram operations.

use xwt_core::utils::maybe;

use crate::types::*;

impl<T> xwt_core::session::datagram::Receive for Session<T>
where
    T: xwt_core::session::datagram::Receive + maybe::Send + maybe::Sync,
{
    type Datagram = Datagram<T::Datagram>;
    type Error = T::Error;

    async fn receive_datagram(&self) -> Result<Self::Datagram, Self::Error> {
        T::receive_datagram(&self.0).await.map(Datagram)
    }
}

impl<T> xwt_core::session::datagram::ReceiveInto for Session<T>
where
    T: xwt_core::session::datagram::ReceiveInto + maybe::Send + maybe::Sync,
{
    type Error = T::Error;

    async fn receive_datagram_into(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        T::receive_datagram_into(&self.0, buf).await
    }
}

impl<T> xwt_core::session::datagram::Send for Session<T>
where
    T: xwt_core::session::datagram::Send + maybe::Send + maybe::Sync,
{
    type Error = T::Error;

    async fn send_datagram<D>(&self, payload: D) -> Result<(), Self::Error>
    where
        D: xwt_core::utils::maybe::Send + AsRef<[u8]>,
    {
        T::send_datagram(&self.0, payload).await
    }
}

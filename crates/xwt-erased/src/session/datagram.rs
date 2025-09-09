#![allow(missing_docs)]

use alloc::boxed::Box;

use crate::Session;
pub struct IncomingDatagram(Box<dyn xwt_dyn::session::datagram::IncomingDatagram>);

impl core::ops::Deref for IncomingDatagram {
    type Target = dyn xwt_dyn::session::datagram::IncomingDatagram;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl AsRef<[u8]> for IncomingDatagram {
    fn as_ref(&self) -> &[u8] {
        (*self.0).as_ref()
    }
}

impl Session {
    pub async fn receive_datagram(&self) -> Result<IncomingDatagram, crate::Error> {
        self.0
            .receive_datagram()
            .await
            .map(IncomingDatagram)
            .map_err(crate::Error::from_inner)
    }

    pub async fn receive_datagram_into(&self, buf: &mut [u8]) -> Result<usize, crate::Error> {
        self.0
            .receive_datagram_into(buf)
            .await
            .map_err(crate::Error::from_inner)
    }

    pub async fn send_datagram<D>(&self, payload: D) -> Result<(), crate::Error>
    where
        D: xwt_dyn::utils::maybe::Send + AsRef<[u8]>,
    {
        self.0
            .send_datagram(payload.as_ref())
            .await
            .map_err(crate::Error::from_inner)
    }

    pub fn max_datagram_size(&self) -> Option<usize> {
        self.0.max_datagram_size()
    }
}

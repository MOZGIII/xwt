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

    pub async fn send_datagram(&self, payload: &[u8]) -> Result<(), crate::Error> {
        self.0
            .send_datagram(payload)
            .await
            .map_err(crate::Error::from_inner)
    }
}

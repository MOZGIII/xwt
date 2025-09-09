static_assertions::assert_impl_all!(crate::Session: xwt_core::base::Session);

impl xwt_core::session::datagram::MaxSize for crate::Session {
    fn max_datagram_size(&self) -> Option<usize> {
        self.max_datagram_size()
    }
}

impl xwt_core::session::datagram::Receive for crate::Session {
    type Datagram = crate::session::datagram::IncomingDatagram;
    type Error = crate::Error;

    async fn receive_datagram(&self) -> Result<Self::Datagram, Self::Error> {
        self.receive_datagram().await
    }
}

impl xwt_core::session::datagram::ReceiveInto for crate::Session {
    type Error = crate::Error;

    async fn receive_datagram_into(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.receive_datagram_into(buf).await
    }
}

impl xwt_core::session::datagram::Send for crate::Session {
    type Error = crate::Error;

    async fn send_datagram<D>(&self, payload: D) -> Result<(), Self::Error>
    where
        D: xwt_dyn::utils::maybe::Send + AsRef<[u8]>,
    {
        self.send_datagram(payload).await
    }
}

impl xwt_core::session::stream::SendSpec for crate::Session {
    type SendStream = crate::stream::SendStream;
}

impl xwt_core::session::stream::RecvSpec for crate::Session {
    type RecvStream = crate::stream::RecvStream;
}

impl xwt_core::session::stream::AcceptBi for crate::Session {
    type Error = crate::Error;

    async fn accept_bi(&self) -> Result<xwt_core::session::stream::TupleFor<Self>, Self::Error> {
        self.accept_bi().await
    }
}

impl xwt_core::session::stream::AcceptUni for crate::Session {
    type Error = crate::Error;

    async fn accept_uni(&self) -> Result<Self::RecvStream, Self::Error> {
        self.accept_uni().await
    }
}

impl xwt_core::session::stream::OpenBi for crate::Session {
    type Error = crate::Error;
    type Opening = crate::session::stream::OpeningBi;

    async fn open_bi(&self) -> Result<Self::Opening, Self::Error> {
        self.open_bi().await
    }
}

impl xwt_core::session::stream::OpenUni for crate::Session {
    type Error = crate::Error;
    type Opening = crate::session::stream::OpeningUni;

    async fn open_uni(&self) -> Result<Self::Opening, Self::Error> {
        self.open_uni().await
    }
}

impl xwt_core::session::stream::OpeningBi for crate::session::stream::OpeningBi {
    type Error = crate::Error;
    type Streams = crate::Session;

    async fn wait_bi(
        self,
    ) -> Result<xwt_core::session::stream::TupleFor<Self::Streams>, Self::Error> {
        self.wait_bi().await
    }
}

impl xwt_core::session::stream::OpeningUni for crate::session::stream::OpeningUni {
    type Error = crate::Error;
    type Streams = crate::Session;

    async fn wait_uni(
        self,
    ) -> Result<<Self::Streams as xwt_core::session::stream::SendSpec>::SendStream, Self::Error>
    {
        self.wait_uni().await
    }
}

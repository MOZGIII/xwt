impl xwt_core::stream::Read for crate::stream::RecvStream {
    type Error = crate::StreamError;

    async fn read(&mut self, buf: &mut [u8]) -> Result<core::num::NonZeroUsize, Self::Error> {
        self.read(buf).await
    }
}

impl xwt_core::stream::ReadAbort for crate::stream::RecvStream {
    type Error = crate::Error;

    async fn abort(self, error_code: xwt_dyn::stream::ErrorCode) -> Result<(), Self::Error> {
        self.abort(error_code).await
    }
}

impl xwt_core::stream::ReadAborted for crate::stream::RecvStream {
    type Error = crate::Error;

    async fn aborted(self) -> Result<xwt_dyn::stream::ErrorCode, Self::Error> {
        self.aborted().await
    }
}

impl xwt_core::stream::Finished for crate::stream::RecvStream {
    type Error = crate::StreamError;

    async fn finished(self) -> Result<(), Self::Error> {
        self.finished().await
    }
}

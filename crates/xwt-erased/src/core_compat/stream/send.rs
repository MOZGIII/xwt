impl xwt_core::stream::Write for crate::stream::SendStream {
    type Error = crate::StreamError;

    async fn write(&mut self, buf: &[u8]) -> Result<core::num::NonZeroUsize, Self::Error> {
        self.write(buf).await
    }
}

impl xwt_core::stream::WriteAbort for crate::stream::SendStream {
    type Error = crate::Error;

    async fn abort(self, error_code: xwt_dyn::stream::ErrorCode) -> Result<(), Self::Error> {
        self.abort(error_code).await
    }
}

impl xwt_core::stream::WriteAborted for crate::stream::SendStream {
    type Error = crate::Error;

    async fn aborted(self) -> Result<xwt_dyn::stream::ErrorCode, Self::Error> {
        self.aborted().await
    }
}

impl xwt_core::stream::Finish for crate::stream::SendStream {
    type Error = crate::Error;

    async fn finish(self) -> Result<(), Self::Error> {
        self.finish().await
    }
}

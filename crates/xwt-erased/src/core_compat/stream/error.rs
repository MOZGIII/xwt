impl xwt_core::stream::ErrorAsErrorCode for crate::stream::Error {
    fn as_error_code(&self) -> Option<xwt_dyn::stream::ErrorCode> {
        self.as_error_code()
    }
}

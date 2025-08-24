use core::num::NonZeroUsize;

use alloc::boxed::Box;

use xwt_dyn::prelude::AsErrorCodeExt;

pub struct OpError(Box<dyn xwt_dyn::stream::OpError>);

impl core::ops::Deref for OpError {
    type Target = dyn xwt_dyn::stream::OpError;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl core::fmt::Debug for OpError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OpError").field(&self.0).finish()
    }
}

impl core::fmt::Display for OpError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.0, f)
    }
}

impl core::error::Error for OpError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        self.0.source()
    }
}

impl OpError {
    pub fn as_error_code_value(&self) -> Option<u32> {
        self.0.as_error_code_value()
    }

    pub fn is_error_code(&self, expected_error_code: u32) -> bool {
        self.0.is_error_code(expected_error_code)
    }

    pub fn is_closed(&self) -> bool {
        self.0.is_closed()
    }
}

pub struct SendStream(Box<dyn xwt_dyn::session::stream::SendStream>);

pub struct RecvStream(Box<dyn xwt_dyn::session::stream::RecvStream>);

impl RecvStream {
    pub const fn from_inner(inner: Box<dyn xwt_dyn::session::stream::RecvStream>) -> Self {
        Self(inner)
    }

    pub async fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<NonZeroUsize, OpError> {
        xwt_dyn::stream::Read::read(&mut *self.0, buf)
            .await
            .map_err(OpError)
    }

    pub async fn abort(self, error_code: u32) -> Result<(), crate::Error> {
        xwt_dyn::stream::ReadAbort::abort(self.0, error_code)
            .await
            .map_err(crate::Error::from_inner)
    }
}

impl SendStream {
    pub const fn from_inner(inner: Box<dyn xwt_dyn::session::stream::SendStream>) -> Self {
        Self(inner)
    }

    pub async fn write<'a>(&'a mut self, buf: &'a [u8]) -> Result<NonZeroUsize, OpError> {
        xwt_dyn::stream::Write::write(&mut *self.0, buf)
            .await
            .map_err(OpError)
    }

    pub async fn abort(self, error_code: u32) -> Result<(), crate::Error> {
        xwt_dyn::stream::WriteAbort::abort(self.0, error_code)
            .await
            .map_err(crate::Error::from_inner)
    }

    pub async fn finish(self) -> Result<(), crate::Error> {
        xwt_dyn::stream::Finish::finish(self.0)
            .await
            .map_err(crate::Error::from_inner)
    }

    pub async fn aborted(
        self,
    ) -> Result<Result<u32, Box<dyn core::any::Any + 'static>>, crate::Error> {
        xwt_dyn::stream::WriteAborted::aborted(self.0)
            .await
            .map_err(crate::Error::from_inner)
    }
}

use core::num::NonZeroUsize;

use alloc::boxed::Box;

use xwt_dyn::prelude::*;

pub struct Error(xwt_dyn::stream::BoxedError<'static>);

impl core::ops::Deref for Error {
    type Target = dyn xwt_dyn::stream::Error;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Error").field(&self.0).finish()
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.0, f)
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        self.0.source()
    }
}

impl Error {
    pub fn as_error_code(&self) -> Option<xwt_dyn::stream::ErrorCode> {
        self.0.as_error_code()
    }

    pub fn is_error_code(&self, expected_error_code: xwt_dyn::stream::ErrorCode) -> bool {
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

    pub async fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> Result<NonZeroUsize, Error> {
        xwt_dyn::stream::Read::read(&mut *self.0, buf)
            .await
            .map_err(Error)
    }

    pub async fn abort(self, error_code: xwt_dyn::stream::ErrorCode) -> Result<(), crate::Error> {
        xwt_dyn::stream::ReadAbort::abort(self.0, error_code)
            .await
            .map_err(crate::Error::from_inner)
    }
}

impl SendStream {
    pub const fn from_inner(inner: Box<dyn xwt_dyn::session::stream::SendStream>) -> Self {
        Self(inner)
    }

    pub async fn write<'a>(&'a mut self, buf: &'a [u8]) -> Result<NonZeroUsize, Error> {
        xwt_dyn::stream::Write::write(&mut *self.0, buf)
            .await
            .map_err(Error)
    }

    pub async fn abort(self, error_code: xwt_dyn::stream::ErrorCode) -> Result<(), crate::Error> {
        xwt_dyn::stream::WriteAbort::abort(self.0, error_code)
            .await
            .map_err(crate::Error::from_inner)
    }

    pub async fn finish(self) -> Result<(), crate::Error> {
        xwt_dyn::stream::Finish::finish(self.0)
            .await
            .map_err(crate::Error::from_inner)
    }

    pub async fn aborted(self) -> Result<xwt_dyn::stream::ErrorCode, crate::Error> {
        xwt_dyn::stream::WriteAborted::aborted(self.0)
            .await
            .map_err(crate::Error::from_inner)
    }
}

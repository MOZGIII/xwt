//! Error type.

use xwt_dyn::utils::traits::maybe_send_sync;

/// An abstract error.
pub struct Error(maybe_send_sync::BoxedError<'static>);

impl Error {
    pub fn into_inner(self) -> maybe_send_sync::BoxedError<'static> {
        self.0
    }

    pub const fn from_inner(inner: maybe_send_sync::BoxedError<'static>) -> Self {
        Self(inner)
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

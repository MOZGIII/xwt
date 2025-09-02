//! A common WebTransport interface for browser and native.
//! Write once, run anywhere.
//!
//! Users should utilise the traits from this crate directly for the most
//! flexibility at the type level.

#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod base;
pub mod endpoint;
pub mod session;
pub mod stream;
pub mod stream_utils;

pub mod utils {
    //! Useful utilities.

    pub mod dummy;
    pub mod maybe;

    pub use core::error::Error;
}

pub mod prelude {
    //! A prelude of the ferquently used types.

    pub use crate::base::Session as _;
    pub use crate::endpoint::accept::{Accept as _, Accepting as _, Request as _};
    pub use crate::endpoint::connect::{Connect as _, Connecting as _};
    pub use crate::session::base::{DatagramOps as _, StreamOps as _};
    pub use crate::session::datagram::{MaxSize as _, Receive as _, ReceiveInto as _, Send as _};
    pub use crate::session::stream::{
        AcceptBi as _, AcceptUni as _, OpenBi as _, OpenUni as _, OpeningBi as _, OpeningUni as _,
    };
    pub use crate::stream::{
        ErrorAsErrorCode as _, Finish as _, Read as _, ReadAbort as _, ReadChunk as _, Write as _,
        WriteAbort as _, WriteAborted as _, WriteChunk as _,
    };

    pub use crate::endpoint::accept_utils::*;
    pub use crate::endpoint::connect_utils::*;
    pub use crate::session::datagram_utils::*;
    pub use crate::session::stream_utils::*;
    pub use crate::stream_utils::*;
}

/// An backward-compat [`Connection`] trait.
#[deprecated = "use base::Session instead"]
pub trait Connection: base::Session {}

#[allow(deprecated)]
impl<T> Connection for T where T: base::Session {}

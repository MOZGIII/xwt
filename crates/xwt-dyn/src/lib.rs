#![no_std]
#![allow(missing_docs, clippy::missing_docs_in_private_items)]
#![allow(clippy::type_complexity)]

extern crate alloc;

pub mod base;
pub mod endpoint;
pub mod session;
pub mod stream;
pub mod stream_utils;

pub mod utils {
    //! Useful utilities.

    pub mod traits;
    pub use xwt_core::utils::maybe;
}

pub mod prelude {
    //! A prelude of the frequently used types.

    pub use crate::base::Session as _;
    pub use crate::endpoint::accept::{Accept as _, Accepting as _, Request as _};
    pub use crate::endpoint::connect::{Connect as _, Connecting as _};
    pub use crate::session::base::{DatagramOps as _, StreamOps as _};
    pub use crate::session::datagram::{Receive as _, ReceiveInto as _, Send as _};
    pub use crate::session::stream::{
        AcceptBi as _, AcceptUni as _, OpenBi as _, OpenUni as _, OpeningBi as _, OpeningUni as _,
    };
    pub use crate::stream::{
        AsErrorCode as _, Finish as _, Read as _, ReadAbort as _, ReadChunkU8 as _, Write as _,
        WriteAbort as _, WriteAborted as _, WriteChunkU8 as _,
    };

    pub use crate::stream_utils::*;

    pub use crate::session::datagram::IncomingDatagram;
    pub use crate::session::stream::{RecvStream, SendStream};

    pub use crate::stream::OpError;
    pub use crate::utils::traits::maybe_send_sync::Error;
}

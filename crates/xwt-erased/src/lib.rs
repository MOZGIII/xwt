#![no_std]
#![allow(missing_docs, clippy::missing_docs_in_private_items)]
#![allow(clippy::type_complexity)]

extern crate alloc;

pub mod endpoint;
pub mod error;
pub mod session;
pub mod stream;

pub use self::endpoint::Client;
pub use self::endpoint::Server;
pub use self::error::Error;
pub use self::session::Session;
pub use self::stream::Error as StreamError;

pub mod utils {
    pub use xwt_dyn::utils::maybe;
}

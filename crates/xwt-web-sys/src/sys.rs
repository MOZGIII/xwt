//! WASM bindings.
//!
//! Since WebTransport bindings in [`web_sys`] are currently locked behind
//! `--cfg=web_sys_unstable_apis`, the bindings that we use are copied in
//! directly from [`web_sys`], so that we can use them without the `cfg`.
//! This also means that downstream dependents don't have to specify this `cfg`
//! flag when building.

mod bidirectional_stream;
mod datagram;
mod options;
mod stream;
mod webtransport;

pub use {bidirectional_stream::*, datagram::*, options::*, stream::*, webtransport::*};

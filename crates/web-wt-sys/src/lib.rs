//! Hand-crafted low-level Web API bindings for WebTransport.

#![cfg(target_family = "wasm")]
#![no_std]

mod macros;
mod streams;
mod webtransport;

pub use streams::*;
pub use webtransport::*;

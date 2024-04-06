//! WebAPI WebTransport low-level bindings.
//!
//! Based on the [WebTransport IDL][idl].
//!
//! [idl]: https://w3c.github.io/webtransport/#idl-index

#![no_std]

/// Generated bindings.
#[allow(missing_docs, clippy::missing_docs_in_private_items, unsafe_code)]
mod gen;

pub use gen::*;

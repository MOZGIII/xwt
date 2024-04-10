//! [`WebTransportReliabilityMode`]
//!
//! <https://w3c.github.io/webtransport/#enumdef-webtransportreliabilitymode>

#![allow(missing_docs)]

use wasm_bindgen::prelude::*;

/// The `WebTransportReliabilityMode` enum.
///
/// <https://w3c.github.io/webtransport/#enumdef-webtransportreliabilitymode>
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebTransportReliabilityMode {
    /// Pending more info.
    Pending = "pending",
    /// Transport only supports reliable transport through uni/bidirectional
    /// streams.
    ReliableOnly = "reliable-only",
    /// Transport supports streams and unreliable datagrams.
    SupportsUnreliable = "supports-unreliable",
}

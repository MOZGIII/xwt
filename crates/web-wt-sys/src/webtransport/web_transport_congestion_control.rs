//! [`WebTransportCongestionControl`]
//!
//! <https://w3c.github.io/webtransport/#enumdef-webtransportcongestioncontrol>

#![allow(missing_docs)]

use wasm_bindgen::prelude::*;

/// The `WebTransportCongestionControl` enum.
///
/// <https://w3c.github.io/webtransport/#enumdef-webtransportcongestioncontrol>
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebTransportCongestionControl {
    /// Default.
    Default = "default",
    /// Favour throughput.
    Throughput = "throughput",
    /// Favour low latency.
    LowLatency = "low-latency",
}

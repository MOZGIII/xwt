#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `WebTransportCongestionControl` enum."]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebTransportCongestionControl {
    Default = "default",
    Throughput = "throughput",
    LowLatency = "low-latency",
}

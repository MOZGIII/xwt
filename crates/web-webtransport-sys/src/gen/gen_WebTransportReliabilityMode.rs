#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `WebTransportReliabilityMode` enum."]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebTransportReliabilityMode {
    Pending = "pending",
    ReliableOnly = "reliable-only",
    SupportsUnreliable = "supports-unreliable",
}

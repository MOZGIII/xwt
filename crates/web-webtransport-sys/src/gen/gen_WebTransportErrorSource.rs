#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `WebTransportErrorSource` enum."]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebTransportErrorSource {
    Stream = "stream",
    Session = "session",
}

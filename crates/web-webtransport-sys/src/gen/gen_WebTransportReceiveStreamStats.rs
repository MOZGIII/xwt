#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportReceiveStreamStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportReceiveStreamStats` dictionary."]
    pub type WebTransportReceiveStreamStats;
    #[wasm_bindgen(method, setter = "bytesRead")]
    fn bytes_read_shim(this: &WebTransportReceiveStreamStats, val: f64);
    #[wasm_bindgen(method, setter = "bytesReceived")]
    fn bytes_received_shim(this: &WebTransportReceiveStreamStats, val: f64);
}
impl WebTransportReceiveStreamStats {
    #[doc = "Construct a new `WebTransportReceiveStreamStats`."]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bytesRead` field of this object."]
    pub fn bytes_read(&mut self, val: f64) -> &mut Self {
        self.bytes_read_shim(val);
        self
    }
    #[doc = "Change the `bytesReceived` field of this object."]
    pub fn bytes_received(&mut self, val: f64) -> &mut Self {
        self.bytes_received_shim(val);
        self
    }
}
impl Default for WebTransportReceiveStreamStats {
    fn default() -> Self {
        Self::new()
    }
}

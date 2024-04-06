#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportSendStreamStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportSendStreamStats` dictionary."]
    pub type WebTransportSendStreamStats;
    #[wasm_bindgen(method, setter = "bytesAcknowledged")]
    fn bytes_acknowledged_shim(this: &WebTransportSendStreamStats, val: f64);
    #[wasm_bindgen(method, setter = "bytesSent")]
    fn bytes_sent_shim(this: &WebTransportSendStreamStats, val: f64);
    #[wasm_bindgen(method, setter = "bytesWritten")]
    fn bytes_written_shim(this: &WebTransportSendStreamStats, val: f64);
}
impl WebTransportSendStreamStats {
    #[doc = "Construct a new `WebTransportSendStreamStats`."]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bytesAcknowledged` field of this object."]
    pub fn bytes_acknowledged(&mut self, val: f64) -> &mut Self {
        self.bytes_acknowledged_shim(val);
        self
    }
    #[doc = "Change the `bytesSent` field of this object."]
    pub fn bytes_sent(&mut self, val: f64) -> &mut Self {
        self.bytes_sent_shim(val);
        self
    }
    #[doc = "Change the `bytesWritten` field of this object."]
    pub fn bytes_written(&mut self, val: f64) -> &mut Self {
        self.bytes_written_shim(val);
        self
    }
}
impl Default for WebTransportSendStreamStats {
    fn default() -> Self {
        Self::new()
    }
}

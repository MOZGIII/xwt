#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportDatagramStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportDatagramStats` dictionary."]
    pub type WebTransportDatagramStats;
    #[wasm_bindgen(method, setter = "droppedIncoming")]
    fn dropped_incoming_shim(this: &WebTransportDatagramStats, val: f64);
    #[wasm_bindgen(method, setter = "expiredOutgoing")]
    fn expired_outgoing_shim(this: &WebTransportDatagramStats, val: f64);
    #[wasm_bindgen(method, setter = "lostOutgoing")]
    fn lost_outgoing_shim(this: &WebTransportDatagramStats, val: f64);
}
impl WebTransportDatagramStats {
    #[doc = "Construct a new `WebTransportDatagramStats`."]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `droppedIncoming` field of this object."]
    pub fn dropped_incoming(&mut self, val: f64) -> &mut Self {
        self.dropped_incoming_shim(val);
        self
    }
    #[doc = "Change the `expiredOutgoing` field of this object."]
    pub fn expired_outgoing(&mut self, val: f64) -> &mut Self {
        self.expired_outgoing_shim(val);
        self
    }
    #[doc = "Change the `lostOutgoing` field of this object."]
    pub fn lost_outgoing(&mut self, val: f64) -> &mut Self {
        self.lost_outgoing_shim(val);
        self
    }
}
impl Default for WebTransportDatagramStats {
    fn default() -> Self {
        Self::new()
    }
}

#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportSendStreamOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportSendStreamOptions` dictionary."]
    pub type WebTransportSendStreamOptions;
    #[wasm_bindgen(method, setter = "sendOrder")]
    fn send_order_shim(this: &WebTransportSendStreamOptions, val: Option<f64>);
}
impl WebTransportSendStreamOptions {
    #[doc = "Construct a new `WebTransportSendStreamOptions`."]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `sendOrder` field of this object."]
    pub fn send_order(&mut self, val: Option<f64>) -> &mut Self {
        self.send_order_shim(val);
        self
    }
}
impl Default for WebTransportSendStreamOptions {
    fn default() -> Self {
        Self::new()
    }
}

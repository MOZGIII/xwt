#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportCloseInfo)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportCloseInfo` dictionary."]
    pub type WebTransportCloseInfo;
    #[wasm_bindgen(method, setter = "closeCode")]
    fn close_code_shim(this: &WebTransportCloseInfo, val: u32);
    #[wasm_bindgen(method, setter = "reason")]
    fn reason_shim(this: &WebTransportCloseInfo, val: &str);
}
impl WebTransportCloseInfo {
    #[doc = "Construct a new `WebTransportCloseInfo`."]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `closeCode` field of this object."]
    pub fn close_code(&mut self, val: u32) -> &mut Self {
        self.close_code_shim(val);
        self
    }
    #[doc = "Change the `reason` field of this object."]
    pub fn reason(&mut self, val: &str) -> &mut Self {
        self.reason_shim(val);
        self
    }
}
impl Default for WebTransportCloseInfo {
    fn default() -> Self {
        Self::new()
    }
}

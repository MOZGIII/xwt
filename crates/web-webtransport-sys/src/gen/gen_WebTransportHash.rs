#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportHash)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportHash` dictionary."]
    pub type WebTransportHash;
    #[wasm_bindgen(method, setter = "algorithm")]
    fn algorithm_shim(this: &WebTransportHash, val: &str);
    #[wasm_bindgen(method, setter = "value")]
    fn value_shim(this: &WebTransportHash, val: &::js_sys::Object);
}
impl WebTransportHash {
    #[doc = "Construct a new `WebTransportHash`."]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `algorithm` field of this object."]
    pub fn algorithm(&mut self, val: &str) -> &mut Self {
        self.algorithm_shim(val);
        self
    }
    #[doc = "Change the `value` field of this object."]
    pub fn value(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.value_shim(val);
        self
    }
}
impl Default for WebTransportHash {
    fn default() -> Self {
        Self::new()
    }
}

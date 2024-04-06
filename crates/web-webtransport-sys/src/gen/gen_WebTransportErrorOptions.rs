#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportErrorOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportErrorOptions` dictionary."]
    pub type WebTransportErrorOptions;
    #[wasm_bindgen(method, setter = "source")]
    fn source_shim(this: &WebTransportErrorOptions, val: WebTransportErrorSource);
    #[wasm_bindgen(method, setter = "streamErrorCode")]
    fn stream_error_code_shim(this: &WebTransportErrorOptions, val: Option<u8>);
}
impl WebTransportErrorOptions {
    #[doc = "Construct a new `WebTransportErrorOptions`."]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `source` field of this object."]
    pub fn source(&mut self, val: WebTransportErrorSource) -> &mut Self {
        self.source_shim(val);
        self
    }
    #[doc = "Change the `streamErrorCode` field of this object."]
    pub fn stream_error_code(&mut self, val: Option<u8>) -> &mut Self {
        self.stream_error_code_shim(val);
        self
    }
}
impl Default for WebTransportErrorOptions {
    fn default() -> Self {
        Self::new()
    }
}

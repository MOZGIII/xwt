#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportOptions` dictionary."]
    pub type WebTransportOptions;
    #[wasm_bindgen(method, setter = "allowPooling")]
    fn allow_pooling_shim(this: &WebTransportOptions, val: bool);
    #[wasm_bindgen(method, setter = "congestionControl")]
    fn congestion_control_shim(this: &WebTransportOptions, val: WebTransportCongestionControl);
    #[wasm_bindgen(method, setter = "requireUnreliable")]
    fn require_unreliable_shim(this: &WebTransportOptions, val: bool);
    #[wasm_bindgen(method, setter = "serverCertificateHashes")]
    fn server_certificate_hashes_shim(this: &WebTransportOptions, val: &::wasm_bindgen::JsValue);
}
impl WebTransportOptions {
    #[doc = "Construct a new `WebTransportOptions`."]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `allowPooling` field of this object."]
    pub fn allow_pooling(&mut self, val: bool) -> &mut Self {
        self.allow_pooling_shim(val);
        self
    }
    #[doc = "Change the `congestionControl` field of this object."]
    pub fn congestion_control(&mut self, val: WebTransportCongestionControl) -> &mut Self {
        self.congestion_control_shim(val);
        self
    }
    #[doc = "Change the `requireUnreliable` field of this object."]
    pub fn require_unreliable(&mut self, val: bool) -> &mut Self {
        self.require_unreliable_shim(val);
        self
    }
    #[doc = "Change the `serverCertificateHashes` field of this object."]
    pub fn server_certificate_hashes(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.server_certificate_hashes_shim(val);
        self
    }
}
impl Default for WebTransportOptions {
    fn default() -> Self {
        Self::new()
    }
}

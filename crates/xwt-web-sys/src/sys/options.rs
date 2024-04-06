//! Bindings for the transport options.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportOptions` dictionary."]
    pub type WebTransportOptions;
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
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("allowPooling"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `congestionControl` field of this object."]
    pub fn congestion_control(&mut self, val: WebTransportCongestionControl) -> &mut Self {
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("congestionControl"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `requireUnreliable` field of this object."]
    pub fn require_unreliable(&mut self, val: bool) -> &mut Self {
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("requireUnreliable"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `serverCertificateHashes` field of this object."]
    pub fn server_certificate_hashes(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("serverCertificateHashes"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}

impl Default for WebTransportOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
#[doc = "The `WebTransportCongestionControl` enum."]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebTransportCongestionControl {
    /// Default.
    Default = "default",
    /// Favour throughput.
    Throughput = "throughput",
    /// Favour low latency.
    LowLatency = "low-latency",
}

#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportSendGroup , typescript_type = "WebTransportSendGroup")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportSendGroup` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendGroup)"]
    pub type WebTransportSendGroup;
    # [wasm_bindgen (method , structural , js_class = "WebTransportSendGroup" , js_name = getStats)]
    #[doc = "The `getStats()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendGroup/getStats)"]
    pub fn get_stats(this: &WebTransportSendGroup) -> ::js_sys::Promise;
}

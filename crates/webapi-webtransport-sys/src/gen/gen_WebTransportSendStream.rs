#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportSendStream , typescript_type = "WebTransportSendStream")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportSendStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendStream)"]
    pub type WebTransportSendStream;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransportSendStream" , js_name = sendGroup)]
    #[doc = "Getter for the `sendGroup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendStream/sendGroup)"]
    pub fn send_group(this: &WebTransportSendStream) -> Option<WebTransportSendGroup>;
    # [wasm_bindgen (structural , method , setter , js_class = "WebTransportSendStream" , js_name = sendGroup)]
    #[doc = "Setter for the `sendGroup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendStream/sendGroup)"]
    pub fn set_send_group(this: &WebTransportSendStream, value: Option<&WebTransportSendGroup>);
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransportSendStream" , js_name = sendOrder)]
    #[doc = "Getter for the `sendOrder` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendStream/sendOrder)"]
    pub fn send_order(this: &WebTransportSendStream) -> f64;
    # [wasm_bindgen (structural , method , setter , js_class = "WebTransportSendStream" , js_name = sendOrder)]
    #[doc = "Setter for the `sendOrder` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendStream/sendOrder)"]
    pub fn set_send_order(this: &WebTransportSendStream, value: f64);
    # [wasm_bindgen (method , structural , js_class = "WebTransportSendStream" , js_name = getStats)]
    #[doc = "The `getStats()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendStream/getStats)"]
    pub fn get_stats(this: &WebTransportSendStream) -> ::js_sys::Promise;
}

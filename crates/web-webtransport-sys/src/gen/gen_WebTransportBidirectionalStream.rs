#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportBidirectionalStream , typescript_type = "WebTransportBidirectionalStream")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportBidirectionalStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportBidirectionalStream)"]
    pub type WebTransportBidirectionalStream;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransportBidirectionalStream" , js_name = readable)]
    #[doc = "Getter for the `readable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportBidirectionalStream/readable)"]
    pub fn readable(this: &WebTransportBidirectionalStream) -> WebTransportReceiveStream;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransportBidirectionalStream" , js_name = writable)]
    #[doc = "Getter for the `writable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportBidirectionalStream/writable)"]
    pub fn writable(this: &WebTransportBidirectionalStream) -> WebTransportSendStream;
}

#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportDatagramDuplexStream , typescript_type = "WebTransportDatagramDuplexStream")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportDatagramDuplexStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream)"]
    pub type WebTransportDatagramDuplexStream;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransportDatagramDuplexStream" , js_name = maxDatagramSize)]
    #[doc = "Getter for the `maxDatagramSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/maxDatagramSize)"]
    pub fn max_datagram_size(this: &WebTransportDatagramDuplexStream) -> u32;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransportDatagramDuplexStream" , js_name = incomingMaxAge)]
    #[doc = "Getter for the `incomingMaxAge` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/incomingMaxAge)"]
    pub fn incoming_max_age(this: &WebTransportDatagramDuplexStream) -> f64;
    # [wasm_bindgen (structural , method , setter , js_class = "WebTransportDatagramDuplexStream" , js_name = incomingMaxAge)]
    #[doc = "Setter for the `incomingMaxAge` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/incomingMaxAge)"]
    pub fn set_incoming_max_age(this: &WebTransportDatagramDuplexStream, value: f64);
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransportDatagramDuplexStream" , js_name = outgoingMaxAge)]
    #[doc = "Getter for the `outgoingMaxAge` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/outgoingMaxAge)"]
    pub fn outgoing_max_age(this: &WebTransportDatagramDuplexStream) -> f64;
    # [wasm_bindgen (structural , method , setter , js_class = "WebTransportDatagramDuplexStream" , js_name = outgoingMaxAge)]
    #[doc = "Setter for the `outgoingMaxAge` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/outgoingMaxAge)"]
    pub fn set_outgoing_max_age(this: &WebTransportDatagramDuplexStream, value: f64);
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransportDatagramDuplexStream" , js_name = incomingHighWaterMark)]
    #[doc = "Getter for the `incomingHighWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/incomingHighWaterMark)"]
    pub fn incoming_high_water_mark(this: &WebTransportDatagramDuplexStream) -> f64;
    # [wasm_bindgen (structural , method , setter , js_class = "WebTransportDatagramDuplexStream" , js_name = incomingHighWaterMark)]
    #[doc = "Setter for the `incomingHighWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/incomingHighWaterMark)"]
    pub fn set_incoming_high_water_mark(this: &WebTransportDatagramDuplexStream, value: f64);
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransportDatagramDuplexStream" , js_name = outgoingHighWaterMark)]
    #[doc = "Getter for the `outgoingHighWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/outgoingHighWaterMark)"]
    pub fn outgoing_high_water_mark(this: &WebTransportDatagramDuplexStream) -> f64;
    # [wasm_bindgen (structural , method , setter , js_class = "WebTransportDatagramDuplexStream" , js_name = outgoingHighWaterMark)]
    #[doc = "Setter for the `outgoingHighWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/outgoingHighWaterMark)"]
    pub fn set_outgoing_high_water_mark(this: &WebTransportDatagramDuplexStream, value: f64);
}

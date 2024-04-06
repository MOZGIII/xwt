#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransport , typescript_type = "WebTransport")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransport` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport)"]
    pub type WebTransport;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransport" , js_name = ready)]
    #[doc = "Getter for the `ready` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/ready)"]
    pub fn ready(this: &WebTransport) -> ::js_sys::Promise;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransport" , js_name = reliability)]
    #[doc = "Getter for the `reliability` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/reliability)"]
    pub fn reliability(this: &WebTransport) -> WebTransportReliabilityMode;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransport" , js_name = congestionControl)]
    #[doc = "Getter for the `congestionControl` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/congestionControl)"]
    pub fn congestion_control(this: &WebTransport) -> WebTransportCongestionControl;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransport" , js_name = closed)]
    #[doc = "Getter for the `closed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/closed)"]
    pub fn closed(this: &WebTransport) -> ::js_sys::Promise;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransport" , js_name = draining)]
    #[doc = "Getter for the `draining` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/draining)"]
    pub fn draining(this: &WebTransport) -> ::js_sys::Promise;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransport" , js_name = datagrams)]
    #[doc = "Getter for the `datagrams` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/datagrams)"]
    pub fn datagrams(this: &WebTransport) -> WebTransportDatagramDuplexStream;
    #[wasm_bindgen(catch, constructor, js_class = "WebTransport")]
    #[doc = "The `new WebTransport(..)` constructor, creating a new instance of `WebTransport`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/WebTransport)"]
    pub fn new(url: &str) -> Result<WebTransport, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "WebTransport")]
    #[doc = "The `new WebTransport(..)` constructor, creating a new instance of `WebTransport`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/WebTransport)"]
    pub fn new_with_options(
        url: &str,
        options: &WebTransportOptions,
    ) -> Result<WebTransport, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "WebTransport" , js_name = close)]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/close)"]
    pub fn close(this: &WebTransport);
    # [wasm_bindgen (method , structural , js_class = "WebTransport" , js_name = close)]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/close)"]
    pub fn close_with_close_info(this: &WebTransport, close_info: &WebTransportCloseInfo);
    # [wasm_bindgen (method , structural , js_class = "WebTransport" , js_name = createBidirectionalStream)]
    #[doc = "The `createBidirectionalStream()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/createBidirectionalStream)"]
    pub fn create_bidirectional_stream(this: &WebTransport) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "WebTransport" , js_name = createBidirectionalStream)]
    #[doc = "The `createBidirectionalStream()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/createBidirectionalStream)"]
    pub fn create_bidirectional_stream_with_options(
        this: &WebTransport,
        options: &WebTransportSendStreamOptions,
    ) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "WebTransport" , js_name = createUnidirectionalStream)]
    #[doc = "The `createUnidirectionalStream()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/createUnidirectionalStream)"]
    pub fn create_unidirectional_stream(this: &WebTransport) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "WebTransport" , js_name = createUnidirectionalStream)]
    #[doc = "The `createUnidirectionalStream()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/createUnidirectionalStream)"]
    pub fn create_unidirectional_stream_with_options(
        this: &WebTransport,
        options: &WebTransportSendStreamOptions,
    ) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "WebTransport" , js_name = getStats)]
    #[doc = "The `getStats()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/getStats)"]
    pub fn get_stats(this: &WebTransport) -> ::js_sys::Promise;
}

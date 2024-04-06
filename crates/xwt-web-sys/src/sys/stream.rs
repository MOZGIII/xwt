//! Bindings for byte streams.

use wasm_bindgen::prelude::*;
use web_sys::{ReadableStream, WritableStream};

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = WritableStream , extends = :: js_sys :: Object , js_name = WebTransportSendStream , typescript_type = "WebTransportSendStream")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportSendStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendStream)"]
    pub type WebTransportSendStream;
    # [wasm_bindgen (method , structural , js_class = "WebTransportSendStream" , js_name = getStats)]
    #[doc = "The `getStats()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendStream/getStats)"]
    pub fn get_stats(this: &WebTransportSendStream) -> ::js_sys::Promise;
}

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = ReadableStream , extends = :: js_sys :: Object , js_name = WebTransportReceiveStream , typescript_type = "WebTransportReceiveStream")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportReceiveStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportReceiveStream)"]
    pub type WebTransportReceiveStream;
    # [wasm_bindgen (method , structural , js_class = "WebTransportReceiveStream" , js_name = getStats)]
    #[doc = "The `getStats()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportReceiveStream/getStats)"]
    pub fn get_stats(this: &WebTransportReceiveStream) -> ::js_sys::Promise;
}

#[wasm_bindgen]
extern "C" {
    /// A result returned by [`ReadableStreamDefaultReader.read`][1] or
    /// [`ReadableStreamBYOBReader.read`][2].
    ///
    /// [1]: https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/read
    /// [2]: https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/read
    #[derive(Clone, Debug)]
    pub type ReadableStreamReadResult;

    #[wasm_bindgen(method, getter, js_name = done)]
    pub fn is_done(this: &ReadableStreamReadResult) -> bool;

    #[wasm_bindgen(method, getter, js_name = value)]
    pub fn value(this: &ReadableStreamReadResult) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ReadableStreamDefaultReader , typescript_type = "ReadableStreamDefaultReader")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamDefaultReader` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader)"]
    pub type ReadableStreamDefaultReader;
    # [wasm_bindgen (structural , method , getter , js_class = "ReadableStreamDefaultReader" , js_name = closed)]
    #[doc = "Getter for the `closed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/closed)"]
    pub fn closed(this: &ReadableStreamDefaultReader) -> ::js_sys::Promise;
    #[cfg(feature = "ReadableStream")]
    #[wasm_bindgen(catch, constructor, js_class = "ReadableStreamDefaultReader")]
    #[doc = "The `new ReadableStreamDefaultReader(..)` constructor, creating a new instance of `ReadableStreamDefaultReader`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/ReadableStreamDefaultReader)"]
    pub fn new(stream: &ReadableStream) -> Result<ReadableStreamDefaultReader, JsValue>;
    # [wasm_bindgen (method , structural , js_class = "ReadableStreamDefaultReader" , js_name = read)]
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/read)"]
    pub fn read(this: &ReadableStreamDefaultReader) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "ReadableStreamDefaultReader" , js_name = releaseLock)]
    #[doc = "The `releaseLock()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/releaseLock)"]
    pub fn release_lock(this: &ReadableStreamDefaultReader);
    # [wasm_bindgen (method , structural , js_class = "ReadableStreamDefaultReader" , js_name = cancel)]
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/cancel)"]
    pub fn cancel(this: &ReadableStreamDefaultReader) -> ::js_sys::Promise;
    # [wasm_bindgen (method , structural , js_class = "ReadableStreamDefaultReader" , js_name = cancel)]
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/cancel)"]
    pub fn cancel_with_reason(
        this: &ReadableStreamDefaultReader,
        reason: &::wasm_bindgen::JsValue,
    ) -> ::js_sys::Promise;
}

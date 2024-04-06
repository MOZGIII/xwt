#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportError , typescript_type = "WebTransportError")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError)"]
    pub type WebTransportError;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransportError" , js_name = source)]
    #[doc = "Getter for the `source` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/source)"]
    pub fn source(this: &WebTransportError) -> WebTransportErrorSource;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransportError" , js_name = streamErrorCode)]
    #[doc = "Getter for the `streamErrorCode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/streamErrorCode)"]
    pub fn stream_error_code(this: &WebTransportError) -> Option<u8>;
    #[wasm_bindgen(catch, constructor, js_class = "WebTransportError")]
    #[doc = "The `new WebTransportError(..)` constructor, creating a new instance of `WebTransportError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/WebTransportError)"]
    pub fn new() -> Result<WebTransportError, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "WebTransportError")]
    #[doc = "The `new WebTransportError(..)` constructor, creating a new instance of `WebTransportError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/WebTransportError)"]
    pub fn new_with_message(message: &str) -> Result<WebTransportError, JsValue>;
    #[wasm_bindgen(catch, constructor, js_class = "WebTransportError")]
    #[doc = "The `new WebTransportError(..)` constructor, creating a new instance of `WebTransportError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/WebTransportError)"]
    pub fn new_with_message_and_options(
        message: &str,
        options: &WebTransportErrorOptions,
    ) -> Result<WebTransportError, JsValue>;
}

//! Bindings for the transport itself.
#![allow(missing_docs)]

use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::{DomException, ReadableStream};

use super::*;

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
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransport" , js_name = incomingBidirectionalStreams)]
    #[doc = "Getter for the `incomingBidirectionalStreams` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/incomingBidirectionalStreams)"]
    pub fn incoming_bidirectional_streams(this: &WebTransport) -> ReadableStream;
    # [wasm_bindgen (structural , method , getter , js_class = "WebTransport" , js_name = incomingUnidirectionalStreams)]
    #[doc = "Getter for the `incomingUnidirectionalStreams` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/incomingUnidirectionalStreams)"]
    pub fn incoming_unidirectional_streams(this: &WebTransport) -> ReadableStream;
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

#[wasm_bindgen]
#[allow(missing_docs)]
#[doc = "The `WebTransportReliabilityMode` enum."]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebTransportReliabilityMode {
    /// Pending more info.
    Pending = "pending",
    /// Transport only supports reliable transport through uni/bidirectional streams.
    ReliableOnly = "reliable-only",
    /// Transport supports streams and unreliable datagrams.
    SupportsUnreliable = "supports-unreliable",
}

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportCloseInfo)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportCloseInfo` dictionary."]
    pub type WebTransportCloseInfo;
}

impl WebTransportCloseInfo {
    #[doc = "Construct a new `WebTransportCloseInfo`."]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `closeCode` field of this object."]
    pub fn close_code(&mut self, val: u32) -> &mut Self {
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("closeCode"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `reason` field of this object."]
    pub fn reason(&mut self, val: &str) -> &mut Self {
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("reason"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}

impl Default for WebTransportCloseInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportSendStreamOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportSendStreamOptions` dictionary."]
    pub type WebTransportSendStreamOptions;
}
impl WebTransportSendStreamOptions {
    #[doc = "Construct a new `WebTransportSendStreamOptions`."]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `sendOrder` field of this object."]
    pub fn send_order(&mut self, val: Option<f64>) -> &mut Self {
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("sendOrder"),
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

impl Default for WebTransportSendStreamOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
extern "C" {
    ///The `WebTransportError` interface.
    ///
    /// <https://w3c.github.io/webtransport/#webtransporterror>
    #[wasm_bindgen(extends = DomException, extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type WebTransportError;

    /// ```webidl
    /// constructor(optional DOMString message = "", optional WebTransportErrorOptions options = {});
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransporterror-webtransporterror>
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebTransportError;

    /// ```webidl
    /// readonly attribute WebTransportErrorSource source;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransporterror-source>
    #[wasm_bindgen(method, getter)]
    pub fn source(this: &WebTransportError) -> WebTransportErrorSource;

    /// ```webidl
    /// readonly attribute unsigned long? streamErrorCode;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransporterror-streamerrorcode>
    #[wasm_bindgen(method, getter = streamErrorCode)]
    pub fn stream_error_code(this: &WebTransportError) -> Option<u32>;
}

/// The `WebTransportErrorSource` enum.
///
/// <https://w3c.github.io/webtransport/#enumdef-webtransporterrorsource>
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebTransportErrorSource {
    /// Error originated from the stream.
    Stream = "stream",
    /// Error originated from the session.
    Session = "session",
}

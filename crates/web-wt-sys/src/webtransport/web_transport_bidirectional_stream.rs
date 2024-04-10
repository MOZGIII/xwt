//! [`WebTransportBidirectionalStream`]
//!
//! <https://w3c.github.io/webtransport/#bidirectional-stream>

use js_sys::Object;
use wasm_bindgen::prelude::*;

use super::*;

#[wasm_bindgen]
extern "C" {
    /// The `WebTransportBidirectionalStream` interface.
    ///
    /// <https://w3c.github.io/webtransport/#webtransportbidirectionalstream>
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type WebTransportBidirectionalStream;

    /// ```webidl
    /// readonly attribute WebTransportReceiveStream readable;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportbidirectionalstream-readable>
    #[wasm_bindgen(method, getter)]
    pub fn readable(this: &WebTransportBidirectionalStream) -> WebTransportReceiveStream;

    /// ```webidl
    /// readonly attribute WebTransportSendStream writable;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportbidirectionalstream-writable>
    #[wasm_bindgen(method, getter)]
    pub fn writable(this: &WebTransportBidirectionalStream) -> WebTransportSendStream;
}

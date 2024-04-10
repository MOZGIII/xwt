//! [`WebTransportError`]
//!
//! <https://w3c.github.io/webtransport/#web-transport-error-interface>

use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::DomException;

use super::*;

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

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

    /// Create an error that carries the given options.
    ///
    /// Both Chrome 154 and Firefox 154 implement the earlier revision of
    /// the constructor, which takes the options as its only argument under
    /// the `WebTransportErrorInit` name:
    ///
    /// ```webidl
    /// constructor(optional WebTransportErrorInit init = {});
    /// ```
    ///
    /// Calling them with the message and the options as two arguments - as
    /// the current spec defines - throws a `TypeError`, so this is the only
    /// way to construct an error with a `streamErrorCode` today.
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransporterror-webtransporterror>
    #[wasm_bindgen(constructor)]
    pub fn new_with_init(init: &WebTransportErrorOptions) -> WebTransportError;

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

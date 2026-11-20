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

    /// Create an error with the given message and options, using
    /// the constructor the current spec defines:
    ///
    /// ```webidl
    /// constructor(optional DOMString message = "", optional WebTransportErrorOptions options = {});
    /// ```
    ///
    /// Engines that implement only the earlier revision of the constructor
    /// (see [`Self::new_with_init`]) throw a `TypeError` here; Firefox 160
    /// implements this form.
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransporterror-webtransporterror>
    #[wasm_bindgen(constructor, catch)]
    pub fn new_with_message_and_options(
        message: &str,
        options: &WebTransportErrorOptions,
    ) -> Result<WebTransportError, JsValue>;

    /// Create an error that carries the given options, using the earlier
    /// revision of the constructor, which takes the options as its only
    /// argument under the `WebTransportErrorInit` name:
    ///
    /// ```webidl
    /// constructor(optional WebTransportErrorInit init = {});
    /// ```
    ///
    /// Chrome 154 and Firefox 159 and earlier implement only this form, and
    /// throw a `TypeError` on [`Self::new_with_message_and_options`].
    ///
    /// This revision declares `streamErrorCode` as `[Clamp] octet`, so these
    /// engines only carry codes in `0..=255`; larger codes do not survive.
    ///
    /// Engines that implement the current spec, such as Firefox 160, do not
    /// throw here: they take the options object as the message and ignore
    /// it otherwise, so the resulting error carries no `streamErrorCode`.
    /// Check [`Self::stream_error_code`] on the result when the code matters.
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

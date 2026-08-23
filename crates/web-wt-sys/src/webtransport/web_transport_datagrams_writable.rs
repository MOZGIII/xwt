//! [`WebTransportDatagramsWritable`]
//!
//! <https://w3c.github.io/webtransport/#datagrams-writable>

use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::{DomException, WritableStream};

use super::*;

#[wasm_bindgen]
extern "C" {
    /// The `WebTransportDatagramsWritable` interface.
    ///
    /// A [`WritableStream`] providing outgoing streaming of datagrams.
    ///
    /// <https://w3c.github.io/webtransport/#webtransportdatagramswritable>
    #[wasm_bindgen(extends = WritableStream, extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type WebTransportDatagramsWritable;

    /// ```webidl
    /// attribute WebTransportSendGroup? sendGroup;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramswritable-sendgroup>
    #[wasm_bindgen(method, getter = sendGroup)]
    pub fn send_group(this: &WebTransportDatagramsWritable) -> Option<WebTransportSendGroup>;

    /// ```webidl
    /// attribute WebTransportSendGroup? sendGroup;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramswritable-sendgroup>
    #[wasm_bindgen(method, setter = sendGroup, catch)]
    pub fn set_option_send_group(
        this: &WebTransportDatagramsWritable,
        value: Option<WebTransportSendGroup>,
    ) -> Result<(), DomException>;

    /// ```webidl
    /// attribute long long sendOrder;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramswritable-sendorder>
    #[wasm_bindgen(method, getter = sendOrder)]
    pub fn send_order(this: &WebTransportDatagramsWritable) -> i64;

    /// ```webidl
    /// attribute long long sendOrder;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramswritable-sendorder>
    #[wasm_bindgen(method, setter = sendOrder)]
    pub fn set_send_order(this: &WebTransportDatagramsWritable, value: i64);
}

impl WebTransportDatagramsWritable {
    crate::set_option_accessors_fallible! {
        /// ```webidl
        /// attribute WebTransportSendGroup? sendGroup;
        /// ```
        ///
        /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramswritable-sendgroup>
        send_group: WebTransportSendGroup => DomException
    }
}

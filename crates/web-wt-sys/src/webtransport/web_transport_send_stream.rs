//! [`WebTransportSendStream`]
//!
//! <https://w3c.github.io/webtransport/#send-stream>

use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::{DomException, WritableStream};

use super::*;

#[wasm_bindgen]
extern "C" {
    ///The `WebTransportSendStream` interface.
    ///
    /// <https://w3c.github.io/webtransport/#webtransportsendstream>
    #[wasm_bindgen (extends = WritableStream, extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type WebTransportSendStream;

    /// ```webidl
    /// attribute WebTransportSendGroup? sendGroup;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportsendstream-sendgroup>
    #[wasm_bindgen(method, getter = sendGroup)]
    pub fn send_group(this: &WebTransportSendStream) -> Option<WebTransportSendGroup>;

    /// ```webidl
    /// attribute WebTransportSendGroup? sendGroup;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportsendstream-sendgroup>
    #[wasm_bindgen(method, setter = sendGroup, catch)]
    pub fn set_option_send_group(
        this: &WebTransportSendStream,
        value: Option<WebTransportSendGroup>,
    ) -> Result<(), DomException>;

    /// ```webidl
    /// attribute long long sendOrder;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportsendstream-sendorder>
    #[wasm_bindgen(method, getter = sendOrder)]
    pub fn send_order(this: &WebTransportSendStream) -> i64;

    /// ```webidl
    /// attribute long long sendOrder;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportsendstream-sendorder>
    #[wasm_bindgen(method, setter = sendOrder)]
    pub fn set_send_order(this: &WebTransportSendStream, value: i64);

    /// ```webidl
    /// Promise<WebTransportSendStreamStats> getStats();
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportsendstream-getstats>
    #[wasm_bindgen(method, js_name = getStats)]
    pub async fn get_stats(this: &WebTransportSendStream) -> WebTransportSendStreamStats;

    /// ```webidl
    /// WebTransportWriter getWriter();
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportsendstream-getwriter>
    ///
    /// This method must be implemented in the same manner as getWriter
    /// inherited from WritableStream, except in place of creating
    /// a WritableStreamDefaultWriter, it must instead create
    /// a WebTransportWriter with this.
    #[wasm_bindgen(method, js_name = getWriter, catch)]
    pub fn get_writer(this: &WebTransportSendStream) -> Result<WebTransportWriter, DomException>;
}

impl WebTransportSendStream {
    crate::set_option_accessors_fallible! {
        /// ```webidl
        /// attribute WebTransportSendGroup? sendGroup;
        /// ```
        ///
        /// <https://w3c.github.io/webtransport/#dom-webtransportsendstream-sendgroup>
        send_group: WebTransportSendGroup => DomException
    }
}

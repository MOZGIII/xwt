//! [`WebTransportReceiveStream`]
//!
//! <https://w3c.github.io/webtransport/#receive-stream>

use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::ReadableStream;

use super::*;

#[wasm_bindgen]
extern "C" {
    ///The `WebTransportReceiveStream` interface.
    ///
    /// <https://w3c.github.io/webtransport/#webtransportreceivestream>
    #[wasm_bindgen(extends = ReadableStream, extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type WebTransportReceiveStream;

    /// ```webidl
    /// Promise<WebTransportReceiveStreamStats> getStats();
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportreceivestream-getstats>
    #[wasm_bindgen(method, js_name = getStats)]
    pub async fn get_stats(this: &WebTransportReceiveStream) -> WebTransportReceiveStreamStats;
}

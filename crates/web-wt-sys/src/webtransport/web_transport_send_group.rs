//! [`WebTransportSendGroup`]
//!
//! <https://w3c.github.io/webtransport/#sendGroup>

use super::*;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// <https://w3c.github.io/webtransport/#webtransportsendgroup>
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type WebTransportSendGroup;

    /// ```webidl
    /// Promise<WebTransportSendStreamStats> getStats();
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportsendgroup-getstats>
    ///
    /// Aggregates stats from all WebTransportSendStreams grouped under this
    /// sendGroup, and reports the result asynchronously.
    #[wasm_bindgen(method, js_name = getStats)]
    pub async fn get_stats(this: &WebTransportSendGroup) -> WebTransportSendStreamStats;
}

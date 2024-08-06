//! [`WebTransportSendStreamOptions`]
//!
//! <https://w3c.github.io/webtransport/#uni-stream-options>

#![allow(missing_docs)]

use wasm_bindgen::prelude::*;

use super::*;

crate::dictionary_type! {
    /// ```webidl
    /// dictionary WebTransportSendStreamOptions {
    ///   WebTransportSendGroup? sendGroup = null;
    ///   long long sendOrder = 0;
    ///   boolean waitUntilAvailable = false;
    /// };
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dictdef-webtransportsendstreamoptions>
    pub type WebTransportSendStreamOptions {
        send_group: WebTransportSendGroup => sendGroup
        send_order: i64 => sendOrder
        wait_until_available: bool => waitUntilAvailable
    }
}

//! [`WebTransportSendOptions`]
//!
//! <https://w3c.github.io/webtransport/#dictdef-webtransportsendoptions>

#![allow(missing_docs)]

use wasm_bindgen::prelude::*;

use super::*;

crate::dictionary_type! {
    /// ```webidl
    /// dictionary WebTransportSendOptions {
    ///   WebTransportSendGroup? sendGroup = null;
    ///   long long sendOrder = 0;
    /// };
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dictdef-webtransportsendoptions>
    pub type WebTransportSendOptions {
        send_group: WebTransportSendGroup => sendGroup
        send_order: i64 => sendOrder
    }
}

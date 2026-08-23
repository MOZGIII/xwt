//! [`WebTransportSendStreamOptions`]
//!
//! <https://w3c.github.io/webtransport/#dictdef-webtransportsendstreamoptions>

#![allow(missing_docs)]

use wasm_bindgen::prelude::*;

use super::*;

crate::dictionary_type! {
    /// ```webidl
    /// dictionary WebTransportSendStreamOptions : WebTransportSendOptions {
    ///   boolean waitUntilAvailable = false;
    /// };
    /// ```
    ///
    /// The `sendGroup` and `sendOrder` members are inherited from
    /// [`WebTransportSendOptions`].
    ///
    /// <https://w3c.github.io/webtransport/#dictdef-webtransportsendstreamoptions>
    pub type WebTransportSendStreamOptions {
        send_group: WebTransportSendGroup => sendGroup
        send_order: i64 => sendOrder
        wait_until_available: bool => waitUntilAvailable
    }
}

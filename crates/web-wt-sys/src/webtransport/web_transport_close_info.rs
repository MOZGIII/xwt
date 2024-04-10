//! [`WebTransportCloseInfo`]
//!
//! <https://w3c.github.io/webtransport/#web-transport-close-info>

use wasm_bindgen::prelude::*;

crate::dictionary_type! {
    /// ```webidl
    /// dictionary WebTransportCloseInfo {
    ///   unsigned long closeCode = 0;
    ///   USVString reason = "";
    /// };
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dictdef-webtransportcloseinfo>
    pub type WebTransportCloseInfo {
        close_code: u32 => closeCode
        reason: js_sys::JsString => reason
    }
}

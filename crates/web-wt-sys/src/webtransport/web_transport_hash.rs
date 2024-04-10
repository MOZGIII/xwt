//! [`WebTransportHash`]
//!
//! <https://w3c.github.io/webtransport/#dictdef-webtransporthash>

use wasm_bindgen::prelude::*;

crate::dictionary_type! {
    /// ```webidl
    /// dictionary WebTransportHash {
    ///   DOMString algorithm;
    ///   BufferSource value;
    /// };
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dictdef-webtransporthash>
    pub type WebTransportHash {
        algorithm_raw: js_sys::JsString => algorithm
        value_raw: JsValue => value
    }
}

impl WebTransportHash {
    /// Set the algorithm.
    pub fn set_algorithm(&self, val: &str) {
        self.set_algorithm_raw(val.into())
    }

    /// Set the value.
    pub fn set_value(&self, val: &[u8]) {
        self.set_value_raw(js_sys::Uint8Array::from(val).into())
    }
}

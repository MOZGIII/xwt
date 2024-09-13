//! [`WebTransportErrorOptions`]
//!
//! <https://w3c.github.io/webtransport/#dictdef-webtransporterroroptions>

use wasm_bindgen::prelude::*;

use super::*;

crate::dictionary_type! {
    /// ```webidl
    /// dictionary WebTransportErrorOptions {
    ///   WebTransportErrorSource source = "stream";
    ///   [Clamp] unsigned long? streamErrorCode = null;
    /// };
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dictdef-webtransporterroroptions>
    pub type WebTransportErrorOptions {
        source: WebTransportErrorSource => source
        stream_error_code: u32 => streamErrorCode
    }
}

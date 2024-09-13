//! [`WebTransportErrorSource`]
//!
//! <https://w3c.github.io/webtransport/#enumdef-webtransporterrorsource>

#![allow(missing_docs)]

use wasm_bindgen::prelude::*;

/// The `WebTransportErrorSource` enum.
///
/// <https://w3c.github.io/webtransport/#enumdef-webtransporterrorsource>
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebTransportErrorSource {
    /// Error originated from the stream.
    Stream = "stream",
    /// Error originated from the session.
    Session = "session",
}

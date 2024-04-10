//! [`WebTransportReceiveStreamStats`]
//!
//! <https://w3c.github.io/webtransport/#receive-stream>

use wasm_bindgen::prelude::*;

crate::dictionary_type! {
    /// ```webidl
    /// dictionary WebTransportReceiveStreamStats {
    ///   unsigned long long bytesReceived;
    ///   unsigned long long bytesRead;
    /// };
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dictdef-webtransportreceivestreamstats>
    pub type WebTransportReceiveStreamStats {
        bytes_received: u64 => bytesReceived
        bytes_read: u64 => bytesRead
    }
}

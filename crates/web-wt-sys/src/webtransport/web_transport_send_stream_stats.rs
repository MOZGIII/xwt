//! [`WebTransportSendStreamStats`]
//!
//! <https://w3c.github.io/webtransport/#send-stream-stats>

use wasm_bindgen::prelude::*;

crate::dictionary_type! {
    /// ```webidl
    /// dictionary WebTransportSendStreamStats {
    ///   unsigned long long bytesWritten;
    ///   unsigned long long bytesSent;
    ///   unsigned long long bytesAcknowledged;
    /// };
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dictdef-webtransportsendstreamstats>
    pub type WebTransportSendStreamStats {
        bytes_written: u64 => bytesWritten
        bytes_sent: u64 => bytesSent
        bytes_acknowledged: u64 => bytesAcknowledged
    }
}

//! [`WebTransportDatagramStats`]

use wasm_bindgen::prelude::*;

crate::dictionary_type! {
    /// ```webidl
    /// dictionary WebTransportDatagramStats {
    ///   unsigned long long droppedIncoming;
    ///   unsigned long long expiredIncoming;
    ///   unsigned long long expiredOutgoing;
    ///   unsigned long long lostOutgoing;
    /// };
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dictdef-webtransportdatagramstats>
    pub type WebTransportDatagramStats {
        dropped_incoming: u64 => droppedIncoming
        expired_incoming: u64 => expiredIncoming
        expired_outgoing: u64 => expiredOutgoing
        lost_outgoing: u64 => lostOutgoing
    }
}

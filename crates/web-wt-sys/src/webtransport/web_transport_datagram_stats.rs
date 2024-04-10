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
        dropped_incoming: u16 => droppedIncoming
        expired_incoming: u16 => expiredIncoming
        expired_outgoing: u16 => expiredOutgoing
        lost_outgoing: u16 => lostOutgoing
    }
}

//! [`WebTransportConnectionStats`]

use wasm_bindgen::prelude::*;

use super::*;

crate::dictionary_type! {
    /// ```webidl
    /// dictionary WebTransportConnectionStats {
    ///     unsigned long long bytesSent;
    ///     unsigned long long packetsSent;
    ///     unsigned long long bytesLost;
    ///     unsigned long long packetsLost;
    ///     unsigned long long bytesReceived;
    ///     unsigned long long packetsReceived;
    ///     DOMHighResTimeStamp smoothedRtt;
    ///     DOMHighResTimeStamp rttVariation;
    ///     DOMHighResTimeStamp minRtt;
    ///     WebTransportDatagramStats datagrams;
    ///     unsigned long long? estimatedSendRate;
    ///   };
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#web-transport-connection-stats>
    pub type WebTransportConnectionStats {
        bytes_sent: u16 => bytesSent
        packets_sent: u16 => packetsSent
        bytes_lost: u16 => bytesLost
        packets_lost: u16 => packetsLost
        bytes_received: u16 => bytesReceived
        packets_received: u16 => packetsReceived
        smoothed_rtt: DOMHighResTimeStamp => smoothedRtt
        rtt_variation: DOMHighResTimeStamp => rttVariation
        min_rtt: DOMHighResTimeStamp => minRtt
        datagrams: WebTransportDatagramStats => datagrams
        estimated_send_rate: u16 => estimatedSendRate
    }
}

/// typedef double DOMHighResTimeStamp;
type DOMHighResTimeStamp = f64;

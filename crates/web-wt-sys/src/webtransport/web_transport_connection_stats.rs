//! [`WebTransportConnectionStats`]

use wasm_bindgen::prelude::*;

use super::*;

crate::dictionary_type! {
    /// ```webidl
    /// dictionary WebTransportConnectionStats {
    ///   unsigned long long bytesSent;
    ///   unsigned long long bytesSentOverhead;
    ///   unsigned long long bytesAcknowledged;
    ///   unsigned long long packetsSent;
    ///   unsigned long long bytesLost;
    ///   unsigned long long packetsLost;
    ///   unsigned long long bytesReceived;
    ///   unsigned long long packetsReceived;
    ///   DOMHighResTimeStamp smoothedRtt;
    ///   DOMHighResTimeStamp rttVariation;
    ///   DOMHighResTimeStamp minRtt;
    ///   required WebTransportDatagramStats datagrams;
    ///   unsigned long long? estimatedSendRate = null;
    ///   boolean atSendCapacity = false;
    /// };
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#web-transport-connection-stats>
    pub type WebTransportConnectionStats {
        bytes_sent: u64 => bytesSent
        bytes_sent_overhead: u64 => bytesSentOverhead
        bytes_acknowledged: u64 => bytesAcknowledged
        packets_sent: u64 => packetsSent
        bytes_lost: u64 => bytesLost
        packets_lost: u64 => packetsLost
        bytes_received: u64 => bytesReceived
        packets_received: u64 => packetsReceived
        smoothed_rtt: DOMHighResTimeStamp => smoothedRtt
        rtt_variation: DOMHighResTimeStamp => rttVariation
        min_rtt: DOMHighResTimeStamp => minRtt
        datagrams: WebTransportDatagramStats => datagrams
        estimated_send_rate: u64 => estimatedSendRate
        at_send_capacity: bool => atSendCapacity
    }
}

/// typedef double DOMHighResTimeStamp;
type DOMHighResTimeStamp = f64;

//! [`WebTransportOptions`]
//!
//! <https://w3c.github.io/webtransport/#dictdef-webtransportoptions>

use wasm_bindgen::prelude::*;

extern crate alloc;

use super::*;

crate::dictionary_type! {
    /// ```webidl
    /// dictionary WebTransportOptions {
    ///   boolean allowPooling = false;
    ///   boolean requireUnreliable = false;
    ///   sequence<WebTransportHash> serverCertificateHashes;
    ///   WebTransportCongestionControl congestionControl = "default";
    ///   [EnforceRange] unsigned short? anticipatedConcurrentIncomingUnidirectionalStreams = null;
    ///   [EnforceRange] unsigned short? anticipatedConcurrentIncomingBidirectionalStreams = null;
    /// };
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dictdef-webtransportoptions>
    pub type WebTransportOptions {
        allow_pooling: bool => allowPooling
        require_unreliable: bool => requireUnreliable
        server_certificate_hashes: alloc::vec::Vec<WebTransportHash>  => serverCertificateHashes
        congestion_control: WebTransportCongestionControl => congestionControl
        anticipated_concurrent_incoming_unidirectional_streams: u16 => anticipatedConcurrentIncomingUnidirectionalStreams
        anticipated_concurrent_incoming_bidirectional_streams: u16 => anticipatedConcurrentIncomingBidirectionalStreams
    }
}

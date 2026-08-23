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
    ///   HeadersInit headers = {};
    ///   sequence<WebTransportHash> serverCertificateHashes = [];
    ///   WebTransportCongestionControl congestionControl = "default";
    ///   [EnforceRange] unsigned short? anticipatedConcurrentIncomingUnidirectionalStreams = null;
    ///   [EnforceRange] unsigned short? anticipatedConcurrentIncomingBidirectionalStreams = null;
    ///   sequence<DOMString> protocols = [];
    ///   ReadableStreamType datagramsReadableType;
    /// };
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dictdef-webtransportoptions>
    pub type WebTransportOptions {
        allow_pooling: bool => allowPooling
        require_unreliable: bool => requireUnreliable
        headers_raw: JsValue => headers
        server_certificate_hashes: alloc::vec::Vec<WebTransportHash>  => serverCertificateHashes
        congestion_control: WebTransportCongestionControl => congestionControl
        anticipated_concurrent_incoming_unidirectional_streams: u16 => anticipatedConcurrentIncomingUnidirectionalStreams
        anticipated_concurrent_incoming_bidirectional_streams: u16 => anticipatedConcurrentIncomingBidirectionalStreams
        protocols: alloc::vec::Vec<js_sys::JsString> => protocols
        datagrams_readable_type: web_sys::ReadableStreamType => datagramsReadableType
    }
}

impl WebTransportOptions {
    /// Set the `headers` field from a [`web_sys::Headers`] object.
    ///
    /// The `headers` field is a `HeadersInit`, so it can also be set to
    /// a sequence of header name/value pairs or a string record via
    /// [`Self::set_headers_raw`].
    pub fn set_headers(&self, val: &web_sys::Headers) {
        self.set_headers_raw(val.clone().into())
    }
}

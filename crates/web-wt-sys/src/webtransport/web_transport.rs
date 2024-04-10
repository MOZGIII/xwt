//! [`WebTransport`]
//!
//! <https://w3c.github.io/webtransport/#webtransport>

use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::{DomException, ReadableStream};

use super::*;

#[wasm_bindgen]
extern "C" {
    /// The `WebTransport` interface.
    ///
    /// <https://w3c.github.io/webtransport/#webtransport>
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type WebTransport;

    /// ```webidl
    /// constructor(USVString url, optional WebTransportOptions options = {});
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-webtransport>
    ///
    /// The `new WebTransport(url)` constructor, creating
    /// a new instance of`WebTransport`.
    #[wasm_bindgen(constructor, catch)]
    pub fn new(url: &str) -> Result<WebTransport, DomException>;

    /// ```webidl
    /// constructor(USVString url, optional WebTransportOptions options = {});
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-webtransport>
    ///
    /// The `new WebTransport(url, { ... })` constructor, creating
    /// a new instance of `WebTransport`.
    #[wasm_bindgen(constructor, catch)]
    pub fn new_with_options(
        url: &str,
        options: &WebTransportOptions,
    ) -> Result<WebTransport, DomException>;

    // =====

    /// ```webidl
    /// Promise<WebTransportConnectionStats> getStats();
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-getstats>
    ///
    /// Gathers stats for this WebTransport's underlying connection and reports
    /// the result asynchronously.
    #[wasm_bindgen(method, js_name = getStats)]
    pub async fn get_stats(this: &WebTransport) -> WebTransportConnectionStats;

    /// ```webidl
    /// readonly attribute Promise<undefined> ready;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-ready>
    ///
    /// A promise fulfilled when the associated WebTransport session gets
    /// established, or rejected if the establishment process failed.
    #[wasm_bindgen(method, getter, catch)]
    pub async fn ready(this: &WebTransport) -> Result<(), DomException>;

    /// ```webidl
    /// readonly attribute WebTransportReliabilityMode reliability;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-reliability>
    ///
    /// Whether connection supports unreliable (over UDP) transport or only
    /// reliable (over TCP fallback) transport.
    /// Returns "pending" until a connection has been established.
    #[wasm_bindgen(method, getter)]
    pub fn reliability(this: &WebTransport) -> WebTransportReliabilityMode;

    /// ```webidl
    /// readonly attribute WebTransportCongestionControl congestionControl;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-congestioncontrol>
    ///
    /// The application’s preference, if requested in the constructor, and
    /// satisfied by the user agent, for a congestion control algorithm
    /// optimized for either throughput or low latency
    /// for sending on this connection.
    /// If a preference was requested but not satisfied, then the value
    /// is "default".
    #[wasm_bindgen(method, getter = congestionControl)]
    pub fn congestion_control(this: &WebTransport) -> WebTransportCongestionControl;

    /// ```webidl
    /// [EnforceRange] attribute unsigned short? anticipatedConcurrentIncomingUnidirectionalStreams;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-anticipatedconcurrentincomingunidirectionalstreams>
    ///
    /// Optionally lets an application specify the number of concurrently open
    /// incoming unidirectional streams it anticipates the server creating.
    /// If not null, the user agent SHOULD attempt to reduce future round-trips
    /// by taking `[[AnticipatedConcurrentIncomingUnidirectionalStreams]]` into
    /// consideration in its negotiations with the server.
    #[wasm_bindgen(method, getter = anticipatedConcurrentIncomingUnidirectionalStreams)]
    pub fn anticipated_concurrent_incoming_unidirectional_streams(
        this: &WebTransport,
    ) -> Option<u16>;

    /// ```webidl
    /// [EnforceRange] attribute unsigned short? anticipatedConcurrentIncomingUnidirectionalStreams;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-anticipatedconcurrentincomingunidirectionalstreams>
    ///
    /// Optionally lets an application specify the number of concurrently open
    /// incoming unidirectional streams it anticipates the server creating.
    /// If not null, the user agent SHOULD attempt to reduce future round-trips
    /// by taking `[[AnticipatedConcurrentIncomingUnidirectionalStreams]]` into
    /// consideration in its negotiations with the server.
    #[wasm_bindgen(method, setter = anticipatedConcurrentIncomingUnidirectionalStreams)]
    pub fn set_option_anticipated_concurrent_incoming_unidirectional_streams(
        this: &WebTransport,
        val: Option<u16>,
    );

    /// ```webidl
    /// [EnforceRange] attribute unsigned short? anticipatedConcurrentIncomingBidirectionalStreams;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-anticipatedconcurrentincomingbidirectionalstreams>
    ///
    /// Optionally lets an application specify the number of concurrently open
    /// bidirectional streams it anticipates the server creating.
    /// If not null, the user agent SHOULD attempt to reduce future round-trips
    /// by taking `[[AnticipatedConcurrentIncomingBidirectionalStreams]]` into
    /// consideration in its negotiations with the server.
    #[wasm_bindgen(method, getter = anticipatedConcurrentIncomingBidirectionalStreams)]
    pub fn anticipated_concurrent_incoming_bidirectional_streams(
        this: &WebTransport,
    ) -> Option<u16>;

    /// ```webidl
    /// [EnforceRange] attribute unsigned short? anticipatedConcurrentIncomingBidirectionalStreams;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-anticipatedconcurrentincomingbidirectionalstreams>
    ///
    /// Optionally lets an application specify the number of concurrently open
    /// bidirectional streams it anticipates the server creating.
    /// If not null, the user agent SHOULD attempt to reduce future round-trips
    /// by taking `[[AnticipatedConcurrentIncomingBidirectionalStreams]]` into
    /// consideration in its negotiations with the server.
    #[wasm_bindgen(method, setter = anticipatedConcurrentIncomingBidirectionalStreams)]
    pub fn set_option_anticipated_concurrent_incoming_bidirectional_streams(
        this: &WebTransport,
        val: Option<u16>,
    );

    // =====

    /// ```webidl
    /// readonly attribute Promise<WebTransportCloseInfo> closed;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-closed>
    ///
    /// A promise fulfilled when the associated WebTransport object is closed
    /// gracefully, or rejected when it is closed abruptly or
    /// failed on initialization.
    #[wasm_bindgen(method, getter, catch)]
    pub async fn closed(this: &WebTransport) -> Result<WebTransportCloseInfo, DomException>;

    /// ```webidl
    /// readonly attribute Promise<undefined> draining;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-draining>
    ///
    /// A promise fulfilled when the associated WebTransport session receives
    /// a DRAIN_WEBTRANSPORT_SESSION capsule or a GOAWAY frame.
    #[wasm_bindgen(method, getter)]
    pub async fn draining(this: &WebTransport);

    /// ```webidl
    /// undefined close(optional WebTransportCloseInfo closeInfo = {});
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-close>
    #[wasm_bindgen(method)]
    pub fn close(this: &WebTransport);

    /// ```webidl
    /// undefined close(optional WebTransportCloseInfo closeInfo = {});
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-close>
    #[wasm_bindgen(method, js_name = close)]
    pub fn close_with_info(this: &WebTransport, close_info: &WebTransportCloseInfo);

    // =====

    /// ```webidl
    /// readonly attribute WebTransportDatagramDuplexStream datagrams;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-datagrams>
    ///
    /// A single duplex stream for sending and receiving datagrams over this
    /// session.
    #[wasm_bindgen(method, getter)]
    pub fn datagrams(this: &WebTransport) -> WebTransportDatagramDuplexStream;

    // =====

    /// ```webidl
    /// Promise<WebTransportBidirectionalStream> createBidirectionalStream(
    ///  optional WebTransportSendStreamOptions options = {});
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-createbidirectionalstream>
    ///
    /// Creates a [`WebTransportBidirectionalStream`] object for an outgoing
    /// bidirectional stream.
    /// Note that the mere creation of a stream is not immediately visible
    /// to the peer until it is used to send data.
    #[wasm_bindgen(method, js_name = createBidirectionalStream, catch)]
    pub async fn create_bidirectional_stream(
        this: &WebTransport,
    ) -> Result<WebTransportBidirectionalStream, DomException>;

    /// ```webidl
    /// Promise<WebTransportBidirectionalStream> createBidirectionalStream(
    ///  optional WebTransportSendStreamOptions options = {});
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-createbidirectionalstream>
    ///
    /// Creates a [`WebTransportBidirectionalStream`] object for an outgoing
    /// bidirectional stream.
    /// Note that the mere creation of a stream is not immediately visible
    /// to the peer until it is used to send data.
    #[wasm_bindgen(method, js_name = createBidirectionalStream, catch)]
    pub async fn create_bidirectional_stream_with_options(
        this: &WebTransport,
        options: WebTransportSendStreamOptions,
    ) -> Result<WebTransportBidirectionalStream, DomException>;

    /// ```webidl
    /// /* a ReadableStream of WebTransportBidirectionalStream objects */
    /// readonly attribute ReadableStream incomingBidirectionalStreams;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-incomingbidirectionalstreams>
    ///
    /// Returns a [`ReadableStream`] of [`WebTransportBidirectionalStreams`]
    /// that have been received from the server.
    #[wasm_bindgen(method, getter = incomingBidirectionalStreams)]
    pub fn incoming_bidirectional_streams(this: &WebTransport) -> ReadableStream;

    // =====

    /// ```webidl
    /// Promise<WebTransportSendStream> createUnidirectionalStream(
    ///   optional WebTransportSendStreamOptions options = {});
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-createunidirectionalstream>
    ///
    /// Creates a [`WebTransportSendStream`] for an outgoing unidirectional
    /// stream.
    /// Note that the mere creation of a stream is not immediately visible
    /// to the server until it is used to send data.
    #[wasm_bindgen(method, js_name = createUnidirectionalStream, catch)]
    pub async fn create_unidirectional_stream(
        this: &WebTransport,
    ) -> Result<WebTransportSendStream, DomException>;

    /// ```webidl
    /// Promise<WebTransportSendStream> createUnidirectionalStream(
    ///   optional WebTransportSendStreamOptions options = {});
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-createunidirectionalstream>
    ///
    /// Creates a [`WebTransportSendStream`] for an outgoing unidirectional
    /// stream.
    /// Note that the mere creation of a stream is not immediately visible
    /// to the server until it is used to send data.
    #[wasm_bindgen(method, js_name = createUnidirectionalStream, catch)]
    pub async fn create_unidirectional_stream_with_options(
        this: &WebTransport,
        options: WebTransportSendStreamOptions,
    ) -> Result<WebTransportSendStream, DomException>;

    /// ```webidl
    /// /* a ReadableStream of WebTransportReceiveStream objects */
    /// readonly attribute ReadableStream incomingUnidirectionalStreams;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-incomingunidirectionalstreams>
    ///
    /// A [`ReadableStream`] of unidirectional streams, each represented by
    /// a [`WebTransportReceiveStream`], that have been received
    /// from the server.
    #[wasm_bindgen(method, getter = incomingUnidirectionalStreams)]
    pub fn incoming_unidirectional_streams(this: &WebTransport) -> ReadableStream;

    /// ```webidl
    /// WebTransportSendGroup createSendGroup();
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-createsendgroup>
    ///
    /// Creates a [`WebTransportSendGroup`].
    #[wasm_bindgen(method, js_name = createSendGroup, catch)]
    pub fn create_send_group(this: &WebTransport) -> Result<WebTransportSendGroup, DomException>;

    // =====

    /// ```webidl
    /// static readonly attribute boolean supportsReliableOnly;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransport-supportsreliableonly>
    ///
    /// Returns true if the user agent supports WebTransport sessions over
    /// exclusively reliable connections, otherwise false.
    #[wasm_bindgen(static_method_of = WebTransport, js_name = supportsReliableOnly)]
    pub fn supports_reliable_only() -> bool;
}

impl WebTransport {
    crate::set_option_accessors! {
        /// ```webidl
        /// [EnforceRange] attribute unsigned short? anticipatedConcurrentIncomingUnidirectionalStreams;
        /// ```
        ///
        /// <https://w3c.github.io/webtransport/#dom-webtransport-anticipatedconcurrentincomingunidirectionalstreams>
        ///
        /// Optionally lets an application specify the number of concurrently open
        /// incoming unidirectional streams it anticipates the server creating.
        /// If not null, the user agent SHOULD attempt to reduce future round-trips
        /// by taking `[[AnticipatedConcurrentIncomingUnidirectionalStreams]]` into
        /// consideration in its negotiations with the server.
        anticipated_concurrent_incoming_unidirectional_streams: u16
    }

    crate::set_option_accessors! {
        /// ```webidl
        /// [EnforceRange] attribute unsigned short? anticipatedConcurrentIncomingBidirectionalStreams;
        /// ```
        ///
        /// <https://w3c.github.io/webtransport/#dom-webtransport-anticipatedconcurrentincomingbidirectionalstreams>
        ///
        /// Optionally lets an application specify the number of concurrently open
        /// bidirectional streams it anticipates the server creating.
        /// If not null, the user agent SHOULD attempt to reduce future round-trips
        /// by taking `[[AnticipatedConcurrentIncomingBidirectionalStreams]]` into
        /// consideration in its negotiations with the server.
        anticipated_concurrent_incoming_bidirectional_streams: u16
    }
}

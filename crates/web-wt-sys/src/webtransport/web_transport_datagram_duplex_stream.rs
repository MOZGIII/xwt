//! [`WebTransportDatagramDuplexStream`]
//!
//! <https://w3c.github.io/webtransport/#duplex-stream>

use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::{DomException, ReadableStream};

use super::*;

#[wasm_bindgen]
extern "C" {
    /// The `WebTransportDatagramDuplexStream` interface.
    ///
    /// <https://w3c.github.io/webtransport/#webtransportdatagramduplexstream>
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type WebTransportDatagramDuplexStream;

    /// ```webidl
    /// WebTransportDatagramsWritable createWritable(
    ///     optional WebTransportSendOptions options = {});
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-createwritable>
    ///
    /// Creates a [`WebTransportDatagramsWritable`] for outgoing datagrams.
    #[wasm_bindgen(method, js_name = createWritable, catch)]
    pub fn create_writable(
        this: &WebTransportDatagramDuplexStream,
    ) -> Result<WebTransportDatagramsWritable, DomException>;

    /// ```webidl
    /// WebTransportDatagramsWritable createWritable(
    ///     optional WebTransportSendOptions options = {});
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-createwritable>
    ///
    /// Creates a [`WebTransportDatagramsWritable`] for outgoing datagrams.
    #[wasm_bindgen(method, js_name = createWritable, catch)]
    pub fn create_writable_with_options(
        this: &WebTransportDatagramDuplexStream,
        options: &WebTransportSendOptions,
    ) -> Result<WebTransportDatagramsWritable, DomException>;

    /// ```webidl
    /// readonly attribute ReadableStream readable;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-readable>
    #[wasm_bindgen(method, getter)]
    pub fn readable(this: &WebTransportDatagramDuplexStream) -> ReadableStream;

    // =====

    /// ```webidl
    /// readonly attribute unsigned long maxDatagramSize;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-maxdatagramsize>
    #[wasm_bindgen(method, getter, js_name = maxDatagramSize)]
    pub fn max_datagram_size(this: &WebTransportDatagramDuplexStream) -> u32;

    /// ```webidl
    /// attribute unrestricted double? incomingMaxAge;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-incomingmaxage>
    #[wasm_bindgen(method, getter, js_name = incomingMaxAge)]
    pub fn incoming_max_age(this: &WebTransportDatagramDuplexStream) -> Option<f64>;

    /// ```webidl
    /// attribute unrestricted double? incomingMaxAge;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-incomingmaxage>
    #[wasm_bindgen(method, setter, js_name = incomingMaxAge)]
    pub fn set_option_incoming_max_age(this: &WebTransportDatagramDuplexStream, value: Option<f64>);

    /// ```webidl
    /// attribute unrestricted double? outgoingMaxAge;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-outgoingmaxage>
    #[wasm_bindgen(method, getter, js_name = outgoingMaxAge)]
    pub fn outgoing_max_age(this: &WebTransportDatagramDuplexStream) -> Option<f64>;

    /// ```webidl
    /// attribute unrestricted double? outgoingMaxAge;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-outgoingmaxage>
    #[wasm_bindgen(method, setter, js_name = outgoingMaxAge)]
    pub fn set_option_outgoing_max_age(this: &WebTransportDatagramDuplexStream, value: Option<f64>);

    /// ```webidl
    /// attribute unsigned long incomingMaxBufferedDatagrams;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-incomingmaxbuffereddatagrams>
    #[wasm_bindgen(method, getter, js_name = incomingMaxBufferedDatagrams)]
    pub fn incoming_max_buffered_datagrams(this: &WebTransportDatagramDuplexStream) -> u32;

    /// ```webidl
    /// attribute unsigned long incomingMaxBufferedDatagrams;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-incomingmaxbuffereddatagrams>
    #[wasm_bindgen(method, setter, js_name = incomingMaxBufferedDatagrams)]
    pub fn set_incoming_max_buffered_datagrams(this: &WebTransportDatagramDuplexStream, value: u32);

    /// ```webidl
    /// attribute unsigned long outgoingMaxBufferedDatagrams;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-outgoingmaxbuffereddatagrams>
    #[wasm_bindgen(method, getter, js_name = outgoingMaxBufferedDatagrams)]
    pub fn outgoing_max_buffered_datagrams(this: &WebTransportDatagramDuplexStream) -> u32;

    /// ```webidl
    /// attribute unsigned long outgoingMaxBufferedDatagrams;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-outgoingmaxbuffereddatagrams>
    #[wasm_bindgen(method, setter, js_name = outgoingMaxBufferedDatagrams)]
    pub fn set_outgoing_max_buffered_datagrams(this: &WebTransportDatagramDuplexStream, value: u32);
}

impl WebTransportDatagramDuplexStream {
    crate::set_option_accessors! {
        /// ```webidl
        /// attribute unrestricted double? incomingMaxAge;
        /// ```
        ///
        /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-incomingmaxage>
        incoming_max_age: f64
    }

    crate::set_option_accessors! {
        /// ```webidl
        /// attribute unrestricted double? outgoingMaxAge;
        /// ```
        ///
        /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-outgoingmaxage>
        outgoing_max_age: f64
    }
}

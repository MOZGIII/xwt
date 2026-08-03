//! [`WebTransportDatagramDuplexStream`]
//!
//! <https://w3c.github.io/webtransport/#duplex-stream>

use js_sys::{Function, Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ReadableStream, WritableStream};

#[wasm_bindgen]
extern "C" {
    /// The `WebTransportDatagramDuplexStream` interface.
    ///
    /// <https://w3c.github.io/webtransport/#webtransportdatagramduplexstream>
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type WebTransportDatagramDuplexStream;

    /// ```webidl
    /// readonly attribute ReadableStream readable;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-readable>
    #[wasm_bindgen(method, getter)]
    pub fn readable(this: &WebTransportDatagramDuplexStream) -> ReadableStream;

    /// ```webidl
    /// readonly attribute WritableStream writable;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-writable>
    #[wasm_bindgen(method, getter)]
    pub fn writable(this: &WebTransportDatagramDuplexStream) -> WritableStream;

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
    pub fn outgoing_max_age(this: &WebTransportDatagramDuplexStream) -> f64;

    /// ```webidl
    /// attribute unrestricted double? outgoingMaxAge;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-outgoingmaxage>
    #[wasm_bindgen(method, setter, js_name = outgoingMaxAge)]
    pub fn set_option_outgoing_max_age(this: &WebTransportDatagramDuplexStream, value: Option<f64>);

    /// ```webidl
    /// attribute unrestricted double incomingHighWaterMark;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-incominghighwatermark>
    #[wasm_bindgen(method, getter, js_name = incomingHighWaterMark)]
    pub fn incoming_high_water_mark(this: &WebTransportDatagramDuplexStream) -> f64;

    /// ```webidl
    /// attribute unrestricted double incomingHighWaterMark;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-incominghighwatermark>
    #[wasm_bindgen(method, setter, js_name = incomingHighWaterMark)]
    pub fn set_incoming_high_water_mark(this: &WebTransportDatagramDuplexStream, value: f64);

    /// ```webidl
    /// attribute unrestricted double outgoingHighWaterMark;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-outgoinghighwatermark>
    #[wasm_bindgen(method, getter, js_name = outgoingHighWaterMark)]
    pub fn outgoing_high_water_mark(this: &WebTransportDatagramDuplexStream) -> f64;

    /// ```webidl
    /// attribute unrestricted double outgoingHighWaterMark;
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportdatagramduplexstream-outgoinghighwatermark>
    #[wasm_bindgen(method, setter, js_name = outgoingHighWaterMark)]
    pub fn set_outgoing_high_water_mark(this: &WebTransportDatagramDuplexStream, value: f64);
}

impl WebTransportDatagramDuplexStream {
    /// Creates a writable datagram stream using the current standardized API,
    /// falling back to the older property still exposed by some browsers.
    pub fn compatible_writable(&self) -> Result<WritableStream, JsValue> {
        let create_writable = Reflect::get(self.as_ref(), &JsValue::from_str("createWritable"))?;
        if let Some(create_writable) = create_writable.dyn_ref::<Function>() {
            return create_writable.call0(self.as_ref())?.dyn_into();
        }

        Reflect::get(self.as_ref(), &JsValue::from_str("writable"))?.dyn_into()
    }

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

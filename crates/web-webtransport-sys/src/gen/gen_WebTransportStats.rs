#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportStats` dictionary."]
    pub type WebTransportStats;
    #[wasm_bindgen(method, setter = "bytesReceived")]
    fn bytes_received_shim(this: &WebTransportStats, val: f64);
    #[wasm_bindgen(method, setter = "bytesSent")]
    fn bytes_sent_shim(this: &WebTransportStats, val: f64);
    #[wasm_bindgen(method, setter = "datagrams")]
    fn datagrams_shim(this: &WebTransportStats, val: &WebTransportDatagramStats);
    #[wasm_bindgen(method, setter = "numIncomingStreamsCreated")]
    fn num_incoming_streams_created_shim(this: &WebTransportStats, val: u32);
    #[wasm_bindgen(method, setter = "numOutgoingStreamsCreated")]
    fn num_outgoing_streams_created_shim(this: &WebTransportStats, val: u32);
    #[wasm_bindgen(method, setter = "packetsLost")]
    fn packets_lost_shim(this: &WebTransportStats, val: f64);
    #[wasm_bindgen(method, setter = "packetsReceived")]
    fn packets_received_shim(this: &WebTransportStats, val: f64);
    #[wasm_bindgen(method, setter = "packetsSent")]
    fn packets_sent_shim(this: &WebTransportStats, val: f64);
}
impl WebTransportStats {
    #[doc = "Construct a new `WebTransportStats`."]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bytesReceived` field of this object."]
    pub fn bytes_received(&mut self, val: f64) -> &mut Self {
        self.bytes_received_shim(val);
        self
    }
    #[doc = "Change the `bytesSent` field of this object."]
    pub fn bytes_sent(&mut self, val: f64) -> &mut Self {
        self.bytes_sent_shim(val);
        self
    }
    #[doc = "Change the `datagrams` field of this object."]
    pub fn datagrams(&mut self, val: &WebTransportDatagramStats) -> &mut Self {
        self.datagrams_shim(val);
        self
    }
    #[doc = "Change the `numIncomingStreamsCreated` field of this object."]
    pub fn num_incoming_streams_created(&mut self, val: u32) -> &mut Self {
        self.num_incoming_streams_created_shim(val);
        self
    }
    #[doc = "Change the `numOutgoingStreamsCreated` field of this object."]
    pub fn num_outgoing_streams_created(&mut self, val: u32) -> &mut Self {
        self.num_outgoing_streams_created_shim(val);
        self
    }
    #[doc = "Change the `packetsLost` field of this object."]
    pub fn packets_lost(&mut self, val: f64) -> &mut Self {
        self.packets_lost_shim(val);
        self
    }
    #[doc = "Change the `packetsReceived` field of this object."]
    pub fn packets_received(&mut self, val: f64) -> &mut Self {
        self.packets_received_shim(val);
        self
    }
    #[doc = "Change the `packetsSent` field of this object."]
    pub fn packets_sent(&mut self, val: f64) -> &mut Self {
        self.packets_sent_shim(val);
        self
    }
}
impl Default for WebTransportStats {
    fn default() -> Self {
        Self::new()
    }
}

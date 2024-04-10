//! [`WebTransportWriter`]
//!
//! <https://w3c.github.io/webtransport/#web-transport-writer-interface>

use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::{DomException, WritableStreamDefaultWriter};

#[wasm_bindgen]
extern "C" {
    ///The `WebTransportWriter` interface.
    ///
    /// <https://w3c.github.io/webtransport/#webtransportwriter>
    #[wasm_bindgen(extends = WritableStreamDefaultWriter, extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type WebTransportWriter;

    /// ```webidl
    /// Promise<undefined> atomicWrite(optional any chunk);
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportwriter-atomicwrite>
    ///
    /// The atomicWrite method will reject if the chunk given to it could
    /// not be sent in its entirety within the flow control window that is
    /// current at the time of sending.
    /// This behavior is designed to satisfy niche transactional applications
    /// sensitive to flow control deadlocks ([RFC9308] Section 4.4).
    ///
    /// NOTE: atomicWrite can still reject after sending some data.
    /// Though it provides atomicity with respect to flow control,
    /// other errors may occur.
    /// atomicWrite does not prevent data from being split between packets or
    /// being interleaved with other data.
    /// Only the sender learns if atomicWrite fails due to lack of available
    /// flow control credit.
    ///
    /// NOTE: Atomic writes can still block if queued behind non-atomic writes.
    /// If the atomic write is rejected, everything queued behind it at that
    /// moment will be rejected as well.
    /// Any non-atomic writes rejected in this way will error the stream.
    /// Applications are therefore encouraged to always await atomic writes.
    #[wasm_bindgen(method, js_name = atomicWrite, catch)]
    pub async fn atomic_write(this: &WebTransportWriter) -> Result<(), DomException>;

    /// ```webidl
    /// Promise<undefined> atomicWrite(optional any chunk);
    /// ```
    ///
    /// <https://w3c.github.io/webtransport/#dom-webtransportwriter-atomicwrite>
    ///
    /// The atomicWrite method will reject if the chunk given to it could
    /// not be sent in its entirety within the flow control window that is
    /// current at the time of sending.
    /// This behavior is designed to satisfy niche transactional applications
    /// sensitive to flow control deadlocks ([RFC9308] Section 4.4).
    ///
    /// NOTE: atomicWrite can still reject after sending some data.
    /// Though it provides atomicity with respect to flow control,
    /// other errors may occur.
    /// atomicWrite does not prevent data from being split between packets or
    /// being interleaved with other data.
    /// Only the sender learns if atomicWrite fails due to lack of available
    /// flow control credit.
    ///
    /// NOTE: Atomic writes can still block if queued behind non-atomic writes.
    /// If the atomic write is rejected, everything queued behind it at that
    /// moment will be rejected as well.
    /// Any non-atomic writes rejected in this way will error the stream.
    /// Applications are therefore encouraged to always await atomic writes.
    #[wasm_bindgen(method, js_name = atomicWrite, catch)]
    pub async fn atomic_write_with_chunk(
        this: &WebTransportWriter,
        chunk: &JsValue,
    ) -> Result<(), DomException>;
}

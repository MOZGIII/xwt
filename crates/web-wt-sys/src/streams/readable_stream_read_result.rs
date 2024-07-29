//! [`ReadableStreamReadResult`]

use wasm_bindgen::prelude::*;

crate::dictionary_type! {
    /// ```webidl
    /// dictionary ReadableStreamReadResult {
    ///   any value;
    ///   boolean done;
    /// };
    /// ```
    ///
    /// <https://streams.spec.whatwg.org/#dictdef-readablestreamreadresult>
    pub type ReadableStreamReadResult {
        value: JsValue => value
        done: bool => done
    }
}

impl From<web_sys::ReadableStreamReadResult> for ReadableStreamReadResult {
    fn from(value: web_sys::ReadableStreamReadResult) -> Self {
        value.unchecked_into()
    }
}

impl From<ReadableStreamReadResult> for web_sys::ReadableStreamReadResult {
    fn from(value: ReadableStreamReadResult) -> Self {
        value.unchecked_into()
    }
}

impl ReadableStreamReadResult {
    /// Returns `true` if `done` field exists and set to `true`.
    pub fn is_done(&self) -> bool {
        self.get_done().unwrap_or(false)
    }
}

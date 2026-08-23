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
    ///
    /// The `T` type parameter is the type of the `value` field, matching
    /// the chunk type of the stream the read result was produced by.
    pub type ReadableStreamReadResult<T = JsValue> {
        value: T => value
        done: bool => done
    }
}

impl<T> From<web_sys::ReadableStreamReadResult> for ReadableStreamReadResult<T>
where
    Self: JsCast,
{
    fn from(value: web_sys::ReadableStreamReadResult) -> Self {
        value.unchecked_into()
    }
}

impl<T> From<ReadableStreamReadResult<T>> for web_sys::ReadableStreamReadResult
where
    ReadableStreamReadResult<T>: JsCast,
{
    fn from(value: ReadableStreamReadResult<T>) -> Self {
        value.unchecked_into()
    }
}

impl<T> ReadableStreamReadResult<T>
where
    Self: JsCast,
{
    /// Returns `true` if `done` field exists and set to `true`.
    pub fn is_done(&self) -> bool {
        self.get_done().unwrap_or(false)
    }
}

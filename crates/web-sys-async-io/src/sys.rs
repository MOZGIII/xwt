use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// A result returned by
    /// [`ReadableStreamByobReader.read`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamByobReader/read)
    /// and
    /// [`ReadableStreamDefaultReader.read`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/read).
    #[derive(Clone, Debug)]
    pub type ReadableStreamReaderValue;

    #[wasm_bindgen(method, getter, js_name = done)]
    pub fn is_done(this: &ReadableStreamReaderValue) -> bool;

    #[wasm_bindgen(method, getter, js_name = value)]
    pub fn value(this: &ReadableStreamReaderValue) -> Option<Uint8Array>;
}

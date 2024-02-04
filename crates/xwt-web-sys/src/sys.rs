//! Additional wasm bindings.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// A result returned by [`ReadableStreamDefaultReader.read`][1] or
    /// [`ReadableStreamBYOBReader.read`][2].
    ///
    /// [1]: https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/read
    /// [2]: https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/read
    #[derive(Clone, Debug)]
    pub type ReadableStreamReadResult;

    #[wasm_bindgen(method, getter, js_name = done)]
    pub fn is_done(this: &ReadableStreamReadResult) -> bool;

    #[wasm_bindgen(method, getter, js_name = value)]
    pub fn value(this: &ReadableStreamReadResult) -> JsValue;
}

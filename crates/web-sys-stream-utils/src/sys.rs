use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use web_sys::ReadableStreamGetReaderOptions;

#[wasm_bindgen]
extern "C" {
    /// A local alias for [`web_sys::ReadableStream`] carrying
    /// a `catch`-enabled binding for
    /// [`ReadableStream.getReader`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/getReader),
    /// as the `web_sys` one throws (and thus aborts) when the reader mode is
    /// unsupported by the stream (e.g. requesting a BYOB reader for
    /// a non-byte stream).
    #[derive(Clone, Debug)]
    pub type ReadableStreamWithFallibleGetReader;

    #[wasm_bindgen(catch, method, structural, js_name = getReader)]
    pub fn get_reader_with_options(
        this: &ReadableStreamWithFallibleGetReader,
        options: &ReadableStreamGetReaderOptions,
    ) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    /// A result returned by
    /// [`ReadableStreamDefaultReader.read`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/read).
    #[derive(Clone, Debug)]
    pub type ReadableStreamDefaultReaderValue;

    #[wasm_bindgen(method, getter, js_name = done)]
    pub fn is_done(this: &ReadableStreamDefaultReaderValue) -> bool;

    #[wasm_bindgen(method, getter, js_name = value)]
    pub fn value(this: &ReadableStreamDefaultReaderValue) -> Option<Uint8Array>;
}

#[wasm_bindgen]
extern "C" {
    /// A result returned by
    /// [`ReadableStreamByobReader.read`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamByobReader/read).
    #[derive(Clone, Debug)]
    pub type ReadableStreamByobReaderValue;

    #[wasm_bindgen(method, getter, js_name = done)]
    pub fn is_done(this: &ReadableStreamByobReaderValue) -> bool;

    #[wasm_bindgen(method, getter, js_name = value)]
    pub fn value(this: &ReadableStreamByobReaderValue) -> Option<Uint8Array>;
}

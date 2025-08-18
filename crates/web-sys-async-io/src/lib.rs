#![cfg_attr(target_family = "wasm", doc = "Utilities for [`web_sys`] async I/O.")]
#![cfg_attr(
    not(target_family = "wasm"),
    doc = "Utilities for `web_sys` async I/O."
)]
#![cfg(target_family = "wasm")]
#![allow(missing_docs, clippy::missing_docs_in_private_items)]

use wasm_bindgen_futures::JsFuture;

pub mod reader;
mod sys;
pub mod writer;

pub use self::reader::Reader;
pub use self::writer::Writer;

fn js_value_to_io_error(error: wasm_bindgen::JsValue) -> std::io::Error {
    let err: String = format!("{error:?}");
    std::io::Error::new(std::io::ErrorKind::Other, err)
}

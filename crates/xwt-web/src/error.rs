//! Error type definition.

/// A generic error encapsulating a JS-land value.
#[derive(Debug)]
pub struct Error(pub wasm_bindgen::JsValue);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Debug;
        self.0.fmt(f)
    }
}

impl std::error::Error for Error {}

impl From<wasm_bindgen::JsValue> for Error {
    fn from(value: wasm_bindgen::JsValue) -> Self {
        Self(value)
    }
}

impl From<wasm_bindgen::JsError> for Error {
    fn from(value: wasm_bindgen::JsError) -> Self {
        wasm_bindgen::JsValue::from(value).into()
    }
}

impl From<web_sys::DomException> for Error {
    fn from(value: web_sys::DomException) -> Self {
        wasm_bindgen::JsValue::from(value).into()
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        wasm_bindgen::JsValue::from(value).into()
    }
}

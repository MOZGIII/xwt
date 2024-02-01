//! WebTransport Certificate utils.

/// Represents the following object:
///
/// ```js
/// {
///   "algorithm": "<algorithm>",
///   "value": ArrayBuffer(<value>)
/// }
/// ```
///
/// Use `sha-256` `algorithm` for SHA-256.
#[derive(Debug)]
pub struct CertHashRef<'a> {
    pub algorithm: &'a str,
    pub value: &'a [u8],
}

impl<'a> From<CertHashRef<'a>> for js_sys::Object {
    fn from(cert_hash: CertHashRef<'a>) -> Self {
        let value_buffer = js_sys::Uint8Array::from(cert_hash.value);
        let result = js_sys::Object::new();
        js_sys::Reflect::set(
            &result,
            &wasm_bindgen::JsValue::from_str("algorithm"),
            &wasm_bindgen::JsValue::from_str(cert_hash.algorithm),
        )
        .unwrap();
        js_sys::Reflect::set(
            &result,
            &wasm_bindgen::JsValue::from_str("value"),
            &value_buffer,
        )
        .unwrap();
        result
    }
}

impl<'a> From<CertHashRef<'a>> for wasm_bindgen::JsValue {
    fn from(value: CertHashRef<'a>) -> Self {
        js_sys::Object::from(value).into()
    }
}

/// Create an [`js_sys::Array`] with parameters suitable for passing to
/// the [`web_sys::WebTransportOptions::server_certificate_hashes`] and
/// assign the corresponding value at the passed `options`.
pub fn assign<'a, Iter>(options: &mut web_sys::WebTransportOptions, iter: Iter)
where
    Iter: IntoIterator<Item = CertHashRef<'a>>,
{
    let array: js_sys::Array = iter.into_iter().map(wasm_bindgen::JsValue::from).collect();
    options.server_certificate_hashes(&array);
}

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

impl<'a> From<CertHashRef<'a>> for web_sys::WebTransportHash {
    fn from(cert_hash: CertHashRef<'a>) -> Self {
        let mut wt_hash = Self::new();

        wt_hash.algorithm(cert_hash.algorithm);
        wt_hash.value(&js_sys::Uint8Array::from(cert_hash.value));

        wt_hash
    }
}

impl<'a> From<CertHashRef<'a>> for js_sys::Object {
    fn from(cert_hash: CertHashRef<'a>) -> Self {
        web_sys::WebTransportHash::from(cert_hash).into()
    }
}

impl<'a> From<CertHashRef<'a>> for wasm_bindgen::JsValue {
    fn from(cert_hash: CertHashRef<'a>) -> Self {
        web_sys::WebTransportHash::from(cert_hash).into()
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

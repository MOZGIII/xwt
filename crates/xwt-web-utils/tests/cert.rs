#![cfg(target_family = "wasm")]

use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn does_not_crash() {
    let mut options = web_sys::WebTransportOptions::new();

    xwt_web_utils::cert::assign(
        &mut options,
        [xwt_web_utils::cert::CertHashRef {
            algorithm: "sha-256",
            value: &[0; 32][..],
        }],
    );
}

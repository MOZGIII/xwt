#![cfg(target_family = "wasm")]

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen(start)]
pub fn start() -> Result<(), wasm_bindgen::JsValue> {
    tracing_wasm::set_as_global_default();
    Ok(())
}

fn test_endpoint() -> xwt_web_sys::Endpoint {
    let mut options = web_sys::WebTransportOptions::new();

    let digest = xwt_cert_utils::digest::sha256(xwt_test_assets::CERT);
    let digest = digest.as_ref();
    console_log!("certificate sha256 digest: {digest:02X?}");

    xwt_web_utils::cert::assign(
        &mut options,
        [xwt_web_utils::cert::CertHashRef {
            algorithm: "sha-256",
            value: digest,
        }],
    );

    web_sys::console::log_1(&js_sys::JSON::stringify(options.as_ref()).unwrap());

    xwt_web_sys::Endpoint { options }
}

#[wasm_bindgen_test]
async fn echo_streams() {
    let endpoint = test_endpoint();

    xwt_tests::tests::echo(endpoint, xwt_tests::consts::ECHO_SERVER_URL)
        .await
        .unwrap();
}

#[wasm_bindgen_test]
async fn echo_datagrams() {
    let endpoint = test_endpoint();

    xwt_tests::tests::echo_datagrams(endpoint, xwt_tests::consts::ECHO_SERVER_URL)
        .await
        .unwrap();
}

#[wasm_bindgen_test]
async fn read_small_buf() {
    let endpoint = test_endpoint();

    xwt_tests::tests::read_small_buf::run(endpoint, xwt_tests::consts::ECHO_SERVER_URL)
        .await
        .unwrap();
}

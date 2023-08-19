#![cfg(target_family = "wasm")]

use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn echo_streams() {
    let endpoint = xwebtransport_web_sys::Endpoint::default();

    xwebtransport_tests::tests::echo(endpoint).await.unwrap();
}

#[wasm_bindgen_test]
async fn echo_datagrams() {
    let endpoint = xwebtransport_web_sys::Endpoint::default();

    xwebtransport_tests::tests::echo_datagrams(endpoint)
        .await
        .unwrap();
}

#![cfg(target_family = "wasm")]

use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn main() {
    let endpoint = xwebtransport_web_sys::Endpoint::default();

    xwebtransport_tests::tests::echo(endpoint).await.unwrap();
}

#![cfg(target_family = "wasm")]

use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn main() {
    let endpoint = xwebtransport_web_sys::Endpoint;

    let connection = xwebtransport_tests::utils::connect(
        endpoint,
        "https://echo.webtransport.day",
        &Default::default(),
    )
    .await
    .unwrap();

    xwebtransport_tests::tests::echo(connection).await.unwrap();
}

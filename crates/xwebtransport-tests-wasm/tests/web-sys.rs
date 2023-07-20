#![cfg(target_family = "wasm")]

#[tokio::test]
async fn main() {
    let endpoint = xwebtransport::Endpoint;

    xwebtransport_tests::connect(endpoint, "https://echo.webtransport.day", ())
        .await
        .unwrap()
}

#![cfg(not(target_family = "wasm"))]

#[tokio::test]
async fn main() {
    let endpoint = wtransport::Endpoint::client(
        wtransport::ClientConfig::builder()
            .with_bind_address("[::]:0".parse().unwrap())
            .with_native_certs()
            .build(),
    )
    .unwrap();

    xwebtransport_tests::connect(endpoint, "https://echo.webtransport.day", ())
        .await
        .unwrap()
}

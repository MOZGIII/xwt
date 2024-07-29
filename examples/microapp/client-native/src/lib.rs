//! The native client example for `xwt`.

#![cfg(not(target_family = "wasm"))]

use xwt_core::prelude::*;

/// The main entrypoint.
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_cert_hash = xwt_cert_fingerprint::Sha256::compute_for_der(xwt_test_assets::CERT);

    let cfg = xwt_wtransport::wtransport::ClientConfig::builder()
        .with_bind_default()
        .with_server_certificate_hashes([wtransport::tls::Sha256Digest::new(
            server_cert_hash.into_inner(),
        )])
        .build();
    let endpoint = xwt_wtransport::wtransport::Endpoint::client(cfg)?;
    let endpoint = xwt_wtransport::Endpoint(endpoint);

    let connect_result = endpoint.connect("https://localhost:8080").await;

    let connecting = match connect_result {
        Ok(connecting) => connecting,
        Err(error) => {
            if let wtransport::error::ConnectingError::ConnectionError(_) = &error {
                eprintln!("Note: make sure to run the example server before running the examples")
            }
            return Err(error.into());
        }
    };

    let session = connecting.wait_connect().await?;

    let mut client = xwt_example_client_shared::ExampleClient {
        session,
        nickname: format!("desktop-{}", rand::random::<u16>()),
        chat_write,
    };
    client.run().await;

    Ok(())
}

/// Print the chat text to the stdout.
fn chat_write(text: &str) {
    use std::io::Write as _;
    std::io::stdout().write_all(text.as_bytes()).unwrap()
}

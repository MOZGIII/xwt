//! Native test.

#![cfg(not(target_family = "wasm"))]

/// Test that the client compiles.
#[test]
#[ignore = "we test compilation only"]
fn client() {
    let endpoint = wtransport::Endpoint::client(wtransport::ClientConfig::default()).unwrap();

    let endpoint = xwt_wtransport::Endpoint(endpoint);

    let _client = xwt_erased::Client::new(endpoint);
}

/// Test that the server compiles.
#[test]
#[ignore = "we test compilation only"]
fn server() {
    let identity = wtransport::Identity::new(
        wtransport::tls::CertificateChain::new(vec![]),
        wtransport::tls::PrivateKey::from_der_pkcs8(vec![]),
    );

    let endpoint = wtransport::Endpoint::server(
        wtransport::ServerConfig::builder()
            .with_bind_default(443)
            .with_identity(identity)
            .build(),
    )
    .unwrap();

    let endpoint = xwt_wtransport::Endpoint(endpoint);

    let _server = xwt_erased::Server::new(endpoint);
}

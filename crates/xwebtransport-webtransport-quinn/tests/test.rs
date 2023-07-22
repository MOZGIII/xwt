#![cfg(not(target_family = "wasm"))]

#[tokio::test]
async fn main() {
    let mut root_cert_store = rustls::RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().unwrap() {
        root_cert_store.add(&rustls::Certificate(cert.0)).unwrap();
    }

    let mut tls_config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();

    tls_config.alpn_protocols = vec![webtransport_quinn::ALPN.to_vec()]; // this one is important

    let config = quinn::ClientConfig::new(std::sync::Arc::new(tls_config));

    let mut client = quinn::Endpoint::client("0.0.0.0:0".parse().unwrap()).unwrap();
    client.set_default_client_config(config);

    let endpoint = xwebtransport_webtransport_quinn::Endpoint(client);

    xwebtransport_tests::tests::echo(endpoint).await.unwrap();
}

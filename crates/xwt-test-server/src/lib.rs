//! The implementation of the echo server to use for xwt testing.
//! Not intended to be wasm-compatible.

#![allow(missing_docs)]
#![cfg(not(target_family = "wasm"))]

pub mod handling;
pub mod routes;

#[derive(Default)]
pub struct EndpointParams {
    pub addr: Option<std::net::SocketAddr>,
    pub identity: Option<wtransport::tls::Identity>,
}

pub async fn endpoint(
    params: EndpointParams,
) -> Result<wtransport::Endpoint<wtransport::endpoint::endpoint_side::Server>, std::io::Error> {
    let EndpointParams { addr, identity } = params;

    let identity = identity.unwrap_or_else(|| {
        wtransport::tls::Identity::new(
            wtransport::tls::CertificateChain::single(
                wtransport::tls::Certificate::from_der(xwt_test_assets::CERT.to_vec()).unwrap(),
            ),
            wtransport::tls::PrivateKey::from_der_pkcs8(xwt_test_assets::KEY.to_vec()),
        )
    });

    match identity.certificate_chain().as_ref().first() {
        Some(cert) => {
            let sha256_fingerpint = xwt_cert_fingerprint::Sha256::compute_for_der(cert.der());
            tracing::info!(message = "using tls certificate", %sha256_fingerpint);
        }
        None => tracing::info!(message = "not using tls certificate"),
    };

    let server_config =
        wtransport::ServerConfig::builder()
            .with_bind_address(addr.unwrap_or(std::net::SocketAddr::V4(
                std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), 0),
            )))
            .with_identity(&identity)
            .build();

    let endpoint = wtransport::Endpoint::server(server_config)?;

    Ok(endpoint)
}

pub async fn serve_endpoint(
    endpoint: wtransport::Endpoint<wtransport::endpoint::endpoint_side::Server>,
) -> Result<std::convert::Infallible, std::io::Error> {
    let bind_addr = endpoint.local_addr()?;
    tracing::info!(message = "serving endpoint", %bind_addr);

    loop {
        let incoming_session = endpoint.accept().await;
        tokio::spawn(async move {
            if let Err(error) = serve_incoming_session(incoming_session).await {
                tracing::error!(message = "error while serving incoming session", %error);
            }
        });
    }
}

pub async fn serve_incoming_session(
    incoming_session: wtransport::endpoint::IncomingSession,
) -> Result<(), wtransport::error::ConnectionError> {
    tracing::info!(message = "got an incoming session");
    let session_request = incoming_session.await?;
    handling::route::<routes::Routes>(session_request).await
}

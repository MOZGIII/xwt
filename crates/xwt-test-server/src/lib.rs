//! The implementation of the echo server to use for xwt testing.
//! Not intended to be wasm-compatible.

#![allow(missing_docs)]
#![cfg(not(target_family = "wasm"))]

pub mod echo;
pub mod echo_open_bi;

#[derive(Default)]
pub struct EndpointParams {
    pub addr: Option<std::net::SocketAddr>,
    pub cert: Option<wtransport::tls::Certificate>,
}

pub async fn endpoint(
    params: EndpointParams,
) -> Result<wtransport::Endpoint<wtransport::endpoint::endpoint_side::Server>, std::io::Error> {
    let EndpointParams { addr, cert } = params;

    let cert = cert.unwrap_or_else(|| {
        wtransport::tls::Certificate::new(vec![xwt_test_assets::CERT], xwt_test_assets::KEY)
            .unwrap()
    });

    match cert.certificates().first() {
        Some(cert) => {
            let sha256_fingerpint = xwt_cert_utils::fingerprint::sha256(cert);
            tracing::info!(message = "using tls certificate", %sha256_fingerpint);
        }
        None => tracing::info!(message = "not using tls certificate"),
    };

    let server_config =
        wtransport::ServerConfig::builder()
            .with_bind_address(addr.unwrap_or(std::net::SocketAddr::V4(
                std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), 0),
            )))
            .with_certificate(cert)
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

    let path = session_request.path();

    match path {
        "/echo" => {
            tracing::info!(message = "invoking echo handler");
            self::echo::serve_session_request(session_request).await
        }
        "/echo-open-bi" => {
            tracing::info!(message = "invoking echo open bi handler");
            self::echo_open_bi::serve_session_request(session_request).await
        }
        _ => {
            tracing::info!(message = "rejecting incoming session due to path mismatch");
            session_request.not_found().await;
            Ok(())
        }
    }
}

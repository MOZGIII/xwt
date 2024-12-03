//! The native server example for `xwt`.

#![cfg(not(target_family = "wasm"))]

use std::sync::Arc;

use xwt_core::prelude::*;
use xwt_example_server_shared::ExampleServer;

/// The main entrypoint.
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::]:8080".parse().unwrap();

    let private_key = wtransport::tls::PrivateKey::from_der_pkcs8(xwt_test_assets::KEY.to_vec());
    let cert = wtransport::tls::Certificate::from_der(xwt_test_assets::CERT.to_vec())?;
    let cert_chain = wtransport::tls::CertificateChain::single(cert);
    let identity = wtransport::Identity::new(cert_chain, private_key);

    let cfg = xwt_wtransport::wtransport::ServerConfig::builder()
        .with_bind_address(addr)
        .with_identity(identity)
        .build();
    let endpoint = xwt_wtransport::wtransport::Endpoint::server(cfg)?;
    let endpoint = xwt_wtransport::Endpoint(endpoint);

    let server = xwt_example_server_shared::ExampleServer::default();
    let server = Arc::new(server);

    loop {
        let Some(accepting) = endpoint.accept().await? else {
            break;
        };

        let server = Arc::clone(&server);
        tokio::spawn(async move {
            if let Err(error) = handle(server, accepting).await {
                eprintln!("Handling error: {error}");
            }
        });
    }

    Ok(())
}

/// Handle connecting users.
async fn handle(
    server: Arc<ExampleServer<xwt_wtransport::Connection>>,
    accepting: xwt_wtransport::IncomingSession,
) -> Result<(), Box<dyn std::error::Error>> {
    // Run the handshake on the incoming socket.
    let request = accepting.wait_accept().await?;

    // Verify some of the `wtransport`-specific details on the incoming
    // connection.
    // We have access to it here because this function specifies concrete
    // type for the `accepting` argument; if it was generic, we would have
    // to require additional trait bounds.
    // Access underlying [`wtransport::endpoint::SessionRequest`] from
    // the [`xwt_wtransport::SessionRequest`].
    let session_request: &wtransport::endpoint::SessionRequest = &request.0;
    // Access the underlying type.
    if session_request.path() != "/" {
        request.close(42).await?;
        return Ok(());
    }

    // Approve the WebTransport session and get the connection.
    let connection = request.ok().await?;

    server.handle(connection).await;

    Ok(())
}

//! The main entrypoint of the test echo server used by xwt.
//!
//! The test server runs as the native app and implements a WebTransport server
//! that responds to the interactions by sending the received data back
//! to the sender.

#[cfg(not(target_family = "wasm"))]
#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let addr = envfury::or_parse("ADDR", "127.0.0.1:8080")?;
    let endpoint = xwt_test_echo_server::endpoint(xwt_test_echo_server::EndpointParams {
        addr: Some(addr),
        cert: None,
    })
    .await?;
    xwt_test_echo_server::serve_endpoint(endpoint).await?;

    Ok(())
}

#[cfg(target_family = "wasm")]
fn main() {}

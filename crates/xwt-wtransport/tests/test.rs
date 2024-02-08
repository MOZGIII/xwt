#![cfg(not(target_family = "wasm"))]
#![feature(once_cell_try)]

use std::matches;

fn setup() -> color_eyre::eyre::Result<()> {
    static INIT: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    INIT.get_or_try_init::<_, color_eyre::eyre::Error>(|| {
        tracing_subscriber::fmt::init();
        color_eyre::install()?;
        Ok(())
    })?;
    Ok(())
}

fn test_endpoint() -> xwt_wtransport::Endpoint<wtransport::endpoint::endpoint_side::Client> {
    let mut root_store = wtransport::tls::rustls::RootCertStore::empty();
    root_store.add_parsable_certificates(&[xwt_test_assets::CERT]);

    let mut tls_config = wtransport::tls::rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    tls_config.alpn_protocols = vec!["h3".into()];

    let endpoint = wtransport::Endpoint::client(
        wtransport::ClientConfig::builder()
            .with_bind_address("0.0.0.0:0".parse().unwrap())
            .with_custom_tls(tls_config)
            .build(),
    )
    .unwrap();

    xwt_wtransport::Endpoint(endpoint)
}

#[tokio::test]
async fn streams() -> color_eyre::eyre::Result<()> {
    setup()?;

    let endpoint = test_endpoint();

    xwt_tests::tests::streams::run(endpoint, xwt_tests::consts::ECHO_SERVER_URL).await?;

    Ok(())
}

#[tokio::test]
async fn datagrams() -> color_eyre::eyre::Result<()> {
    setup()?;

    let endpoint = test_endpoint();

    xwt_tests::tests::datagrams::run(endpoint, xwt_tests::consts::ECHO_SERVER_URL).await?;

    Ok(())
}

#[tokio::test]
async fn datagrams_read_into() -> color_eyre::eyre::Result<()> {
    setup()?;

    let endpoint = test_endpoint();

    xwt_tests::tests::datagrams_read_into::run(endpoint, xwt_tests::consts::ECHO_SERVER_URL)
        .await?;

    Ok(())
}

#[tokio::test]
async fn read_small_buf() -> color_eyre::eyre::Result<()> {
    setup()?;

    let endpoint = test_endpoint();

    xwt_tests::tests::read_small_buf::run(endpoint, xwt_tests::consts::ECHO_SERVER_URL).await?;

    Ok(())
}

#[tokio::test]
async fn connection_drop() -> color_eyre::eyre::Result<()> {
    setup()?;

    let endpoint = test_endpoint();

    xwt_tests::tests::connection_drop::run(endpoint, xwt_tests::consts::ECHO_SERVER_URL, |error| {
        matches!(error, wtransport::error::StreamReadError::NotConnected)
    })
    .await?;

    Ok(())
}

#[tokio::test]
async fn accept_bi_stream() -> color_eyre::eyre::Result<()> {
    setup()?;

    let endpoint = test_endpoint();

    xwt_tests::tests::accept_bi_stream::run(endpoint, xwt_tests::consts::ECHO_OPEN_BI_SERVER_URL)
        .await?;

    Ok(())
}

#![cfg(not(target_family = "wasm"))]
#![feature(once_cell_try)]

use std::matches;

static_assertions::assert_impl_all!(xwt_wtransport::Endpoint<wtransport::endpoint::endpoint_side::Client>: xwt_core::endpoint::Connect);
static_assertions::assert_impl_all!(xwt_wtransport::Endpoint<wtransport::endpoint::endpoint_side::Server>: xwt_core::endpoint::Accept);
static_assertions::assert_impl_all!(xwt_wtransport::Session: xwt_core::base::Session);

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
    root_store.add_parsable_certificates(
        [xwt_test_assets::CERT].map(wtransport::tls::rustls::pki_types::CertificateDer::from_slice),
    );

    let digest = xwt_cert_fingerprint::Sha256::compute_for_der(xwt_test_assets::CERT);
    tracing::info!("certificate sha256 digest: {digest}");

    let mut tls_config = wtransport::tls::rustls::ClientConfig::builder()
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
async fn read_resize_buf() -> color_eyre::eyre::Result<()> {
    setup()?;

    let endpoint = test_endpoint();

    xwt_tests::tests::read_resize_buf::run(endpoint, xwt_tests::consts::ECHO_SERVER_URL).await?;

    Ok(())
}

#[tokio::test]
async fn session_drop() -> color_eyre::eyre::Result<()> {
    setup()?;

    let endpoint = test_endpoint();

    xwt_tests::tests::session_drop::run(endpoint, xwt_tests::consts::ECHO_SERVER_URL, |error| {
        matches!(
            error,
            xwt_wtransport::StreamReadError::Read(wtransport::error::StreamReadError::NotConnected)
        )
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

#[tokio::test]
async fn closed_uni_stream() -> color_eyre::eyre::Result<()> {
    setup()?;

    let endpoint = test_endpoint();

    xwt_tests::tests::closed_uni_stream::run(
        endpoint,
        xwt_tests::concat!(xwt_tests::consts::ECHO_CLOSE_SERVER_URL, "/uni"),
        0,
    )
    .await?;

    Ok(())
}

#[tokio::test]
async fn closed_uni_stream_with_error() -> color_eyre::eyre::Result<()> {
    setup()?;

    let endpoint = test_endpoint();

    xwt_tests::tests::closed_uni_stream::run(
        endpoint,
        xwt_tests::concat!(xwt_tests::consts::ECHO_CLOSE_SERVER_URL, "/uni/error"),
        123,
    )
    .await?;

    Ok(())
}

#[tokio::test]
async fn closed_bi_read_stream() -> color_eyre::eyre::Result<()> {
    setup()?;

    let endpoint = test_endpoint();

    xwt_tests::tests::closed_bi_read_stream::run(
        endpoint,
        xwt_tests::concat!(xwt_tests::consts::ECHO_CLOSE_SERVER_URL, "/bi/recv"),
        0,
    )
    .await?;

    Ok(())
}

#[tokio::test]
async fn closed_bi_read_stream_with_error() -> color_eyre::eyre::Result<()> {
    setup()?;

    let endpoint = test_endpoint();

    xwt_tests::tests::closed_bi_read_stream::run(
        endpoint,
        xwt_tests::concat!(xwt_tests::consts::ECHO_CLOSE_SERVER_URL, "/bi/recv/error"),
        123,
    )
    .await?;

    Ok(())
}

#[tokio::test]
async fn closed_bi_send_stream() -> color_eyre::eyre::Result<()> {
    setup()?;

    let endpoint = test_endpoint();

    xwt_tests::tests::closed_bi_send_stream::run(
        endpoint,
        xwt_tests::concat!(xwt_tests::consts::ECHO_CLOSE_SERVER_URL, "/bi/send"),
        0,
    )
    .await?;

    Ok(())
}

#[tokio::test]
async fn closed_bi_send_stream_with_error() -> color_eyre::eyre::Result<()> {
    setup()?;

    let endpoint = test_endpoint();

    xwt_tests::tests::closed_bi_send_stream::run(
        endpoint,
        xwt_tests::concat!(xwt_tests::consts::ECHO_CLOSE_SERVER_URL, "/bi/send/error"),
        123,
    )
    .await?;

    Ok(())
}

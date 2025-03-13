//! Integration tests.

#![cfg(target_family = "wasm")]

use wasm_bindgen_test::*;
use xwt_web::{CertificateHash, HashAlgorithm, WebTransportOptions};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

static_assertions::assert_impl_all!(xwt_web::Endpoint: xwt_core::endpoint::Connect);
static_assertions::assert_impl_all!(xwt_web::Session: xwt_core::base::Session);

fn setup() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        tracing_wasm::set_as_global_default();
    });
}

fn test_endpoint() -> xwt_web::Endpoint {
    let digest = xwt_cert_fingerprint::Sha256::compute_for_der(xwt_test_assets::CERT);
    console_log!("certificate sha256 digest: {digest}");

    let options = WebTransportOptions {
        server_certificate_hashes: vec![CertificateHash {
            algorithm: HashAlgorithm::Sha256,
            value: digest.into_inner().to_vec(),
        }],
        ..Default::default()
    };

    xwt_web::Endpoint {
        options: options.to_js(),
    }
}

#[wasm_bindgen_test]
async fn streams() {
    setup();

    let endpoint = test_endpoint();

    xwt_tests::tests::streams::run(endpoint, xwt_tests::consts::ECHO_SERVER_URL)
        .await
        .unwrap();
}

#[wasm_bindgen_test]
async fn datagrams() {
    setup();

    let endpoint = test_endpoint();

    xwt_tests::tests::datagrams::run(endpoint, xwt_tests::consts::ECHO_SERVER_URL)
        .await
        .unwrap();
}

#[wasm_bindgen_test]
async fn datagrams_read_into() {
    setup();

    let endpoint = test_endpoint();

    xwt_tests::tests::datagrams_read_into::run(endpoint, xwt_tests::consts::ECHO_SERVER_URL)
        .await
        .unwrap();
}

#[wasm_bindgen_test]
async fn read_small_buf() {
    setup();

    let endpoint = test_endpoint();

    xwt_tests::tests::read_small_buf::run(endpoint, xwt_tests::consts::ECHO_SERVER_URL)
        .await
        .unwrap();
}

#[wasm_bindgen_test]
async fn read_resize_buf() {
    setup();

    let endpoint = test_endpoint();

    xwt_tests::tests::read_resize_buf::run(endpoint, xwt_tests::consts::ECHO_SERVER_URL)
        .await
        .unwrap();
}

#[wasm_bindgen_test]
async fn tokio_io() {
    setup();

    let endpoint = test_endpoint();

    xwt_tests::tests::tokio_io::run(endpoint, xwt_tests::consts::ECHO_SERVER_URL)
        .await
        .unwrap();
}

#[wasm_bindgen_test]
async fn tokio_io_read_small_buf() {
    setup();

    let endpoint = test_endpoint();

    xwt_tests::tests::tokio_io_read_small_buf::run(endpoint, xwt_tests::consts::ECHO_SERVER_URL)
        .await
        .unwrap();
}

#[wasm_bindgen_test]
async fn tokio_io_read_buf_resize() {
    setup();

    let endpoint = test_endpoint();

    xwt_tests::tests::tokio_io_read_buf_resize::run(endpoint, xwt_tests::consts::ECHO_SERVER_URL)
        .await
        .unwrap();
}

#[wasm_bindgen_test]
async fn session_drop() {
    setup();

    let endpoint = test_endpoint();

    xwt_tests::tests::session_drop::run(endpoint, xwt_tests::consts::ECHO_SERVER_URL, |error| {
        let known_bad_errors = ["Connection lost."];
        let known_good_errors = ["WebTransportError: The session is closed."];
        let actual_error = error.to_string();

        let is_bad_error = known_bad_errors
            .into_iter()
            .any(|known_bad_error| actual_error.contains(known_bad_error));
        if is_bad_error {
            return false;
        }

        known_good_errors
            .into_iter()
            .any(|is_good_error| actual_error.contains(is_good_error))
    })
    .await
    .unwrap();
}

#[wasm_bindgen_test]
async fn accept_bi_stream() {
    setup();

    let endpoint = test_endpoint();

    xwt_tests::tests::accept_bi_stream::run(endpoint, xwt_tests::consts::ECHO_OPEN_BI_SERVER_URL)
        .await
        .unwrap();
}

#[wasm_bindgen_test]
async fn closed_uni_stream() {
    setup();

    let endpoint = test_endpoint();

    xwt_tests::tests::closed_uni_stream::run(
        endpoint,
        xwt_tests::concat!(xwt_tests::consts::ECHO_CLOSE_SERVER_URL, "/uni"),
        0,
    )
    .await
    .unwrap();
}

#[wasm_bindgen_test]
async fn closed_uni_stream_with_error() {
    setup();

    let endpoint = test_endpoint();

    xwt_tests::tests::closed_uni_stream::run(
        endpoint,
        xwt_tests::concat!(xwt_tests::consts::ECHO_CLOSE_SERVER_URL, "/uni/error"),
        123,
    )
    .await
    .unwrap();
}

#[wasm_bindgen_test]
async fn closed_bi_read_stream() {
    setup();

    let endpoint = test_endpoint();

    xwt_tests::tests::closed_bi_read_stream::run(
        endpoint,
        xwt_tests::concat!(xwt_tests::consts::ECHO_CLOSE_SERVER_URL, "/bi/recv"),
        0,
    )
    .await
    .unwrap();
}

#[wasm_bindgen_test]
async fn closed_bi_read_stream_with_error() {
    setup();

    let endpoint = test_endpoint();

    xwt_tests::tests::closed_bi_read_stream::run(
        endpoint,
        xwt_tests::concat!(xwt_tests::consts::ECHO_CLOSE_SERVER_URL, "/bi/recv/error"),
        123,
    )
    .await
    .unwrap();
}

#[wasm_bindgen_test]
async fn closed_bi_send_stream() {
    setup();

    let endpoint = test_endpoint();

    xwt_tests::tests::closed_bi_send_stream::run(
        endpoint,
        xwt_tests::concat!(xwt_tests::consts::ECHO_CLOSE_SERVER_URL, "/bi/send"),
        0,
    )
    .await
    .unwrap();
}

#[wasm_bindgen_test]
async fn closed_bi_send_stream_with_error() {
    setup();

    let endpoint = test_endpoint();

    xwt_tests::tests::closed_bi_send_stream::run(
        endpoint,
        xwt_tests::concat!(xwt_tests::consts::ECHO_CLOSE_SERVER_URL, "/bi/send/error"),
        123.try_into().unwrap(),
    )
    .await
    .unwrap();
}

#![cfg(target_family = "wasm")]

use wasm_bindgen_test::*;
use xwt_web_sys::{CertificateHash, HashAlgorithm, WebTransportOptions};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

fn setup() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        tracing_wasm::set_as_global_default();
    });
}

fn test_endpoint() -> xwt_web_sys::Endpoint {
    let digest = xwt_cert_utils::digest::sha256(xwt_test_assets::CERT);
    let digest = digest.as_ref();
    console_log!("certificate sha256 digest: {digest:02X?}");

    let options = WebTransportOptions {
        server_certificate_hashes: vec![CertificateHash {
            algorithm: HashAlgorithm::Sha256,
            value: digest.to_vec(),
        }],
        ..Default::default()
    };

    xwt_web_sys::Endpoint {
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
async fn connection_drop() {
    setup();

    let endpoint = test_endpoint();

    xwt_tests::tests::connection_drop::run(endpoint, xwt_tests::consts::ECHO_SERVER_URL, |error| {
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

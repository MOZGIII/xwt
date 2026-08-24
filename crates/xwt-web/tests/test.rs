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

/// Obtain the user agent string.
fn user_agent() -> String {
    js_sys::Reflect::get(&js_sys::global(), &"navigator".into())
        .and_then(|navigator| js_sys::Reflect::get(&navigator, &"userAgent".into()))
        .ok()
        .and_then(|user_agent| user_agent.as_string())
        .unwrap_or_default()
}

/// Detect Firefox from the user agent string.
///
/// Used to skip the tests that exercise the WebTransport behaviors that
/// Firefox does not implement properly yet.
fn is_firefox() -> bool {
    user_agent().contains("Firefox")
}

/// Detect Safari from the user agent string.
///
/// Used to skip the tests that exercise the WebTransport behaviors that
/// Safari does not implement properly yet.
fn is_safari() -> bool {
    // The Chrome user agent contains "Safari" too, so rule it out.
    let user_agent = user_agent();
    user_agent.contains("Safari") && !user_agent.contains("Chrome")
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
async fn tokio_io_read_shrink_buf() {
    setup();

    let endpoint = test_endpoint();

    xwt_tests::tests::tokio_io_read_shrink_buf::run(endpoint, xwt_tests::consts::ECHO_SERVER_URL)
        .await
        .unwrap();
}

#[wasm_bindgen_test]
async fn tokio_io_read_partial_buf() {
    setup();

    let endpoint = test_endpoint();

    xwt_tests::tests::tokio_io_read_partial_buf::run(endpoint, xwt_tests::consts::ECHO_SERVER_URL)
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
        let known_good_errors = [
            // Chrome.
            "WebTransportError: The session is closed.",
            // Firefox reports the reads canceled by the session closure with
            // the name of the operation that caused the cancellation.
            "WebTransportError: close()",
            // Safari reports the reads canceled by the session closure with
            // a generic abort error.
            "AbortError: The operation was aborted.",
        ];
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

    if is_firefox() {
        // Firefox does not reject the writer `closed` promise when the peer
        // sends a `STOP_SENDING`, so waiting for the send stream abortion
        // hangs forever.
        // See:
        // - <https://bugzilla.mozilla.org/show_bug.cgi?id=1986138>
        // - <https://bugzilla.mozilla.org/show_bug.cgi?id=2009530>
        console_log!(
            "skipping this test on Firefox: STOP_SENDING is not propagated to the send stream"
        );
        return;
    }

    if is_safari() {
        // Safari has the same issue as Firefox: the writer `closed` promise
        // is not rejected when the peer sends a `STOP_SENDING` (as required
        // by <https://w3c.github.io/webtransport/#webtransportsendstream>),
        // so waiting for the send stream abortion hangs forever.
        // No dedicated WebKit bug is filed for this as of 2026-08; the spec
        // compliance umbrella issue:
        // - <https://bugs.webkit.org/show_bug.cgi?id=297534>
        console_log!(
            "skipping this test on Safari: STOP_SENDING is not propagated to the send stream"
        );
        return;
    }

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

    if is_firefox() {
        // Firefox does not reject the writer `closed` promise when the peer
        // sends a `STOP_SENDING`, so waiting for the send stream abortion
        // hangs forever.
        // See:
        // - <https://bugzilla.mozilla.org/show_bug.cgi?id=1986138>
        // - <https://bugzilla.mozilla.org/show_bug.cgi?id=2009530>
        console_log!(
            "skipping this test on Firefox: STOP_SENDING is not propagated to the send stream"
        );
        return;
    }

    if is_safari() {
        // Safari has the same issue as Firefox: the writer `closed` promise
        // is not rejected when the peer sends a `STOP_SENDING` (as required
        // by <https://w3c.github.io/webtransport/#webtransportsendstream>),
        // so waiting for the send stream abortion hangs forever.
        // No dedicated WebKit bug is filed for this as of 2026-08; the spec
        // compliance umbrella issue:
        // - <https://bugs.webkit.org/show_bug.cgi?id=297534>
        console_log!(
            "skipping this test on Safari: STOP_SENDING is not propagated to the send stream"
        );
        return;
    }

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

    if is_safari() {
        // Safari does not reject the writer `closed` promise when the peer
        // sends a `STOP_SENDING` (as required by
        // <https://w3c.github.io/webtransport/#webtransportsendstream>),
        // so waiting for the send stream abortion hangs forever.
        // No dedicated WebKit bug is filed for this as of 2026-08; the spec
        // compliance umbrella issue:
        // - <https://bugs.webkit.org/show_bug.cgi?id=297534>
        console_log!(
            "skipping this test on Safari: STOP_SENDING is not propagated to the send stream"
        );
        return;
    }

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

    if is_safari() {
        // Safari does not reject the writer `closed` promise when the peer
        // sends a `STOP_SENDING` (as required by
        // <https://w3c.github.io/webtransport/#webtransportsendstream>),
        // so waiting for the send stream abortion hangs forever.
        // No dedicated WebKit bug is filed for this as of 2026-08; the spec
        // compliance umbrella issue:
        // - <https://bugs.webkit.org/show_bug.cgi?id=297534>
        console_log!(
            "skipping this test on Safari: STOP_SENDING is not propagated to the send stream"
        );
        return;
    }

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

    if is_safari() {
        // Safari does not settle the reads on a stream that was cleanly
        // closed by the peer: the end of stream is never signaled to
        // the reader, so the read hangs forever.
        // No dedicated WebKit bug is filed for this as of 2026-08; the spec
        // compliance umbrella issue:
        // - <https://bugs.webkit.org/show_bug.cgi?id=297534>
        console_log!("skipping this test on Safari: stream FIN is not propagated to the reads");
        return;
    }

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

    if is_firefox() {
        // Firefox rejects the reads on a stream that got a `RESET_STREAM`
        // with a generic `TypeError: Error in input stream` instead of
        // a `WebTransportError` carrying the `streamErrorCode`, so there is
        // no error code to observe.
        // See:
        // - <https://bugzilla.mozilla.org/show_bug.cgi?id=2009530>
        // - <https://bugzilla.mozilla.org/show_bug.cgi?id=2009593>
        console_log!(
            "skipping this test on Firefox: RESET_STREAM error code is not exposed to reads"
        );
        return;
    }

    if is_safari() {
        // Safari does not settle the reads on a stream that got
        // a `RESET_STREAM` at all (as required by
        // <https://w3c.github.io/webtransport/#webtransportreceivestream>),
        // so the read hangs forever.
        // No dedicated WebKit bug is filed for this as of 2026-08; the spec
        // compliance umbrella issue:
        // - <https://bugs.webkit.org/show_bug.cgi?id=297534>
        console_log!("skipping this test on Safari: RESET_STREAM is not propagated to the reads");
        return;
    }

    let endpoint = test_endpoint();

    xwt_tests::tests::closed_bi_send_stream::run(
        endpoint,
        xwt_tests::concat!(xwt_tests::consts::ECHO_CLOSE_SERVER_URL, "/bi/send/error"),
        123.try_into().unwrap(),
    )
    .await
    .unwrap();
}

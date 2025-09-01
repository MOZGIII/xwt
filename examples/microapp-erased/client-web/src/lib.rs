//! The web client example for `xwt`.

#![cfg(target_family = "wasm")]

use wasm_bindgen::prelude::*;
use xwt_core::prelude::*;

/// The main entrypoint.
pub async fn make_client() -> Result<xwt_erased_example_client_shared::ExampleClient, JsValue> {
    let server_cert_hash = xwt_cert_fingerprint::Sha256::compute_for_der(xwt_test_assets::CERT);

    let options = xwt_web::WebTransportOptions {
        server_certificate_hashes: vec![xwt_web::CertificateHash {
            algorithm: xwt_web::HashAlgorithm::Sha256,
            value: server_cert_hash.into_inner().to_vec(),
        }],
        ..Default::default()
    };
    let endpoint = xwt_web::Endpoint {
        options: options.to_js(),
    };

    let connecting = endpoint
        .connect("https://localhost:8080")
        .await
        .expect("make sure to run the example server before running the examples");

    let session = connecting.wait_connect().await.unwrap();

    let session = xwt_erased::Session::from_inner(Box::new(session));

    setup_chat();

    Ok(xwt_erased_example_client_shared::ExampleClient {
        session,
        nickname: format!("web-{}", rand::random::<u16>()),
        chat_write,
    })
}

/// Prepare the HTML element to write the chat text to.
fn setup_chat() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let chat_box = document.create_element("div").unwrap();
    chat_box.set_id("chat");
    chat_box
        .set_attribute(
            "style",
            "white-space: pre; font-family: monospace; line-height: 1.5em",
        )
        .unwrap();

    let body = document.body().unwrap();
    body.append_child(&chat_box).unwrap();
}

/// Append the chat text to the document.
fn chat_write(text: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let chat = document.get_element_by_id("chat").unwrap();
    chat.append_with_str_1(text).unwrap();
    chat.scroll_into_view_with_bool(false);
}

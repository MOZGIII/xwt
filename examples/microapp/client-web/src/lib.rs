//! The web client example for `xwt`.

#![cfg(target_family = "wasm")]

use wasm_bindgen::prelude::*;
use xwt_core::prelude::*;

/// The main entrypoint.
pub async fn main() -> Result<(), JsValue> {
    let server_cert_hash = xwt_cert_fingerprint::Sha256::compute_for_der(xwt_test_assets::CERT);

    let options = xwt_web_sys::WebTransportOptions {
        server_certificate_hashes: vec![xwt_web_sys::CertificateHash {
            algorithm: xwt_web_sys::HashAlgorithm::Sha256,
            value: server_cert_hash.into_inner().to_vec(),
        }],
        ..Default::default()
    };
    let endpoint = xwt_web_sys::Endpoint {
        options: options.to_js(),
    };

    let connecting = endpoint
        .connect("https://localhost:8080")
        .await
        .expect("make sure to run the example server before running the examples");

    let session = connecting.wait_connect().await.unwrap();

    setup_chat();

    let mut client = xwt_example_client_shared::ExampleClient {
        session,
        nickname: format!("web-{}", rand::random::<u16>()),
        chat_write,
    };
    client.run().await;

    Ok(())
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

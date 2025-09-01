//! Web test.

#![cfg(target_family = "wasm")]

/// Test that the client compiles.
#[test]
#[ignore = "we test compilation only"]
fn client() {
    let endpoint = xwt_web::Endpoint {
        options: Default::default(),
    };

    let _client = xwt_erased::Client::new(endpoint);
}

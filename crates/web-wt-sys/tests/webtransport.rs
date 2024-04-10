#![cfg(target_family = "wasm")]

use wasm_bindgen_test::*;
use web_wt_sys::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

fn setup() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        tracing_wasm::set_as_global_default();
    });
}

#[wasm_bindgen_test]
async fn dictionary_macro_rw() {
    setup();

    let opts = WebTransportOptions::new();

    assert!(opts.allow_pooling().is_none());

    opts.set_allow_pooling(true);
    assert!(opts.allow_pooling().unwrap());

    opts.set_allow_pooling(false);
    assert!(!opts.allow_pooling().unwrap());

    opts.unset_allow_pooling();
    assert!(opts.allow_pooling().is_none());
}

//! Integration tests.

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

    assert!(opts.get_allow_pooling().is_none());

    opts.set_allow_pooling(true);
    assert!(opts.get_allow_pooling().unwrap());

    opts.set_allow_pooling(false);
    assert!(!opts.get_allow_pooling().unwrap());

    opts.unset_allow_pooling();
    assert!(opts.get_allow_pooling().is_none());
}

/// Constructs an error with the given stream error code in both constructor
/// forms and logs what each one reads back.
fn construct_both_forms(
    code: u32,
) -> (
    Result<WebTransportError, wasm_bindgen::JsValue>,
    WebTransportError,
) {
    let options = WebTransportErrorOptions::new();
    options.set_source(WebTransportErrorSource::Stream);
    options.set_stream_error_code(code);

    let spec_form = WebTransportError::new_with_message_and_options("", &options);
    let legacy_form = WebTransportError::new_with_init(&options);

    console_log!(
        "code {code}: spec form {}, legacy form reads back {:?}",
        match &spec_form {
            Ok(error) => format!("reads back {:?}", error.stream_error_code()),
            Err(_) => "throws".to_owned(),
        },
        legacy_form.stream_error_code(),
    );

    (spec_form, legacy_form)
}

/// Ensures that at least one of the `WebTransportError` constructor forms
/// produces an error that carries the given stream error code - which is what
/// constructing an abort reason relies on.
///
/// The codes stay within `0..=255`, as the legacy form declares the code as
/// an `octet`, and engines with only that form cannot carry larger codes.
#[wasm_bindgen_test]
async fn error_with_stream_error_code() {
    setup();

    for code in [0, 123, 255] {
        let (spec_form, legacy_form) = construct_both_forms(code);

        let error = match spec_form {
            Ok(error) if error.stream_error_code() == Some(code) => error,
            _ => legacy_form,
        };

        assert_eq!(
            error.stream_error_code(),
            Some(code),
            "no constructor form carries the code {code}"
        );
        assert_eq!(error.source(), WebTransportErrorSource::Stream);
    }
}

/// Ensures that the spec constructor form, where an engine implements it,
/// carries the full 32-bit stream error code range.
#[wasm_bindgen_test]
async fn error_with_large_stream_error_code() {
    setup();

    for code in [256, u32::MAX] {
        let (spec_form, _legacy_form) = construct_both_forms(code);

        if let Ok(error) = spec_form {
            assert_eq!(error.stream_error_code(), Some(code));
            assert_eq!(error.source(), WebTransportErrorSource::Stream);
        }
    }
}

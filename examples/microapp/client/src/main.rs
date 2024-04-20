//! Unified entrypoint for native and web `xwt` example client.
//!
//! The reason to have the unified entrypoint is to have a `bin` target that has
//! a `main` fn on each of the platforms that we test against, so that
//! the compiler doesn't complain about missing `main` in the native bin when
//! we are testing wasm and vice versa.

cfg_if::cfg_if! {
    if #[cfg(target_family = "wasm")] {

        fn main() {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));

            wasm_bindgen_futures::spawn_local(async move {
                xwt_example_client_web::main().await.unwrap()
            })
        }

    } else {

        #[tokio::main]
        async fn main() -> Result<(), Box<dyn std::error::Error>> {
            xwt_example_client_native::main().await
        }

    }
}

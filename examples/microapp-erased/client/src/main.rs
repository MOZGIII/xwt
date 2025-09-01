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
                let client = xwt_erased_example_client_web::make_client().await.unwrap();
                run(client).await;
            })
        }

    } else {

        #[tokio::main]
        async fn main() -> Result<(), Box<dyn std::error::Error>> {
            let client = xwt_erased_example_client_native::make_client().await?;
            run(client).await;
            Ok(())
        }

    }
}

/// Run the client with some common and type-unified logic.
/// This is not possible with just `xwt_core`, as you need to specify
/// a concrete driver type - forcing you to make a decision on which driver
/// you will be using at compile time.
async fn run(mut client: xwt_erased_example_client_shared::ExampleClient) {
    client.run().await;
}

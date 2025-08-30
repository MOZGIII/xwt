//! Unified entrypoint for native `xwt` example server.
//!
//! The reason to have the unified entrypoint is to have a `bin` target that has
//! a `main` fn on each of the platforms that we test against, so that
//! the compiler doesn't complain about missing `main` in the native bin when
//! we are testing wasm and vice versa.

cfg_if::cfg_if! {
    if #[cfg(target_family = "wasm")] {

        fn main() {
            unimplemented!("there is no Web API for the WebTransport server");
        }

    } else {

        #[tokio::main]
        async fn main() -> Result<(), Box<dyn std::error::Error>> {
            xwt_erased_example_server_native::main().await
        }

    }
}

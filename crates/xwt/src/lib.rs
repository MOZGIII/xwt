//! The ╳-platform WebTransport implementation.
//!
//! DEPRECATED: use [`xwt-core`] and manually picked drvers instead.
//!
//! See the examples in the repo on how `xwt` is intended to be used.

cfg_if::cfg_if! {
    if #[cfg(target_family = "wasm")] {

        pub use xwt_web_sys as web_sys;
        pub use self::web_sys as current;

    } else {

        pub use xwt_wtransport as wtransport;
        pub use self::wtransport as current;

    }
}

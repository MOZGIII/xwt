//! A build script to print the deprecation warning.

/// The deprecation message to print during build.
const DEPRECATION_MESSAGE: &str = "\
This crate provides a simple mapping to one of the drivers depending on \
the current build target (native vs wasm). For better portability, \
the `xwt-core` crate should be used instead, and the drivers should \
be hand-picked for each of the targets instead of using a crate like this one. \
We will be removing the `xwt` crate soon, and you are free to import \
`xwt-web-sys` and `xwt-wtransport` manually and put them under \
a `cfg_if!` macro yourself if you prefer this mode of operation; they key \
difference there is you maintain explicit control over the your driver \
dependencies rather than us having to tie them together for you - and this how \
it should be. \
See the examples on Github for more info on how we intend for xwt to be used.\
";

fn main() {
    println!("cargo:warning={DEPRECATION_MESSAGE}");
}

//! Build script to generate the assets.

fn main() {
    xwt_test_assets_build::save(
        xwt_test_assets_build::env_dir("CARGO_MANIFEST_DIR").join("target"),
        xwt_test_assets_build::generate(),
    );
}

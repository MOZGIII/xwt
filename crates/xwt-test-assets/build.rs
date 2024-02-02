//! Build script to generate the assets.

fn main() {
    xwt_test_assets_build::save(
        xwt_test_assets_build::generate(),
        xwt_test_assets_build::state_dir(),
    );
}

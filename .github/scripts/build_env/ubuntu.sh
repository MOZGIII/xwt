#!/bin/bash
set -euo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

get_wasm_bindgen_version() {
  cargo metadata \
    --format-version 1 \
    --manifest-path crates/xwt-web/Cargo.toml \
    --filter-platform wasm32-unknown-unknown \
    --quiet |
    jq -r \
      '.packages[] | select(.name == "wasm-bindgen").version'
}

# Install wasm-bindgen-test-runner.
WASM_BINDGEN_VERSION="$(get_wasm_bindgen_version)"
curl -sSL "https://github.com/rustwasm/wasm-bindgen/releases/download/${WASM_BINDGEN_VERSION}/wasm-bindgen-${WASM_BINDGEN_VERSION}-x86_64-unknown-linux-musl.tar.gz" |
  sudo tar -xzvf - -C /usr/local/bin --strip-components=1

# Install chromedriver and chrome.
CHROME_VERSION="141.0.7376.0"
npx @puppeteer/browsers install "chromedriver@${CHROME_VERSION}"
npx @puppeteer/browsers install "chrome@${CHROME_VERSION}"

# Set CHROMEDRIVER env var.
CHROMEDRIVERS=(chromedriver/*/chromedriver-*/chromedriver)
printf "CHROMEDRIVER=%s\n" "${CHROMEDRIVERS[0]}" >>"$GITHUB_ENV"

# Set CHROMEDRIVER_ARGS env var.
printf "CHROMEDRIVER_ARGS=--log-level=INFO\n" >>"$GITHUB_ENV"

# Prepend chrome dir to PATH.
CHROMES=(chrome/*/chrome-*/chrome)
printf "%s\n" "$(pwd)/$(dirname "${CHROMES[0]}")" >>"$GITHUB_PATH"

# Remove the preinstalled chrome.
rm -rf /opt/google/chrome

source "${SCRIPT_DIR}/common.sh"

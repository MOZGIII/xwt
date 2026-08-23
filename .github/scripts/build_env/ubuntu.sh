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

install_wasm_bindgen_test_runner() {
  local WASM_BINDGEN_VERSION
  WASM_BINDGEN_VERSION="$(get_wasm_bindgen_version)"
  curl -sSL "https://github.com/rustwasm/wasm-bindgen/releases/download/${WASM_BINDGEN_VERSION}/wasm-bindgen-${WASM_BINDGEN_VERSION}-x86_64-unknown-linux-musl.tar.gz" |
    sudo tar -xzvf - -C /usr/local/bin --strip-components=1
}

install_chrome() {
  local VERSION="$1"

  # Install chromedriver and chrome.
  npx @puppeteer/browsers install "chromedriver@${VERSION}"
  npx @puppeteer/browsers install "chrome@${VERSION}"

  # Set CHROMEDRIVER env var.
  local CHROMEDRIVERS=(chromedriver/*/chromedriver-*/chromedriver)
  printf "CHROMEDRIVER=%s\n" "${CHROMEDRIVERS[0]}" >>"$GITHUB_ENV"

  # Set CHROMEDRIVER_ARGS env var.
  printf "CHROMEDRIVER_ARGS=--log-level=INFO\n" >>"$GITHUB_ENV"

  # Prepend chrome dir to PATH.
  local CHROMES=(chrome/*/chrome-*/chrome)
  printf "%s\n" "$(pwd)/$(dirname "${CHROMES[0]}")" >>"$GITHUB_PATH"

  # Remove the preinstalled chrome.
  rm -rf /opt/google/chrome
}

install_firefox() {
  local VERSION="$1"
  local GECKODRIVER_VERSION="$2"

  # Install geckodriver.
  mkdir -p geckodriver
  curl -sSL "https://github.com/mozilla/geckodriver/releases/download/v${GECKODRIVER_VERSION}/geckodriver-v${GECKODRIVER_VERSION}-linux64.tar.gz" |
    tar -xzvf - -C geckodriver

  # Install firefox.
  npx @puppeteer/browsers install "firefox@stable_${VERSION}"

  # Set GECKODRIVER env var.
  printf "GECKODRIVER=%s/geckodriver/geckodriver\n" "$(pwd)" >>"$GITHUB_ENV"

  # Point geckodriver at the installed firefox instead of the preinstalled one.
  local FIREFOXES=(firefox/*/firefox/firefox)
  printf "GECKODRIVER_ARGS=--binary %s/%s\n" "$(pwd)" "${FIREFOXES[0]}" >>"$GITHUB_ENV"
}

# Install the wasm test environment when a wasm test browser is requested.
case "${WASM_TEST_BROWSER:-}" in
chrome)
  install_wasm_bindgen_test_runner
  install_chrome "$WASM_TEST_BROWSER_VERSION"
  ;;
firefox)
  install_wasm_bindgen_test_runner
  install_firefox "$WASM_TEST_BROWSER_VERSION" "$WASM_TEST_GECKODRIVER_VERSION"
  ;;
"")
  # Not running the wasm tests; no browser needed.
  ;;
*)
  printf "Unsupported WASM_TEST_BROWSER: %s\n" "$WASM_TEST_BROWSER" >&2
  exit 1
  ;;
esac

source "${SCRIPT_DIR}/common.sh"

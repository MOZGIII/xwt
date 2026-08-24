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
  local ARCH
  ARCH="$(uname -m)"
  if [[ "$ARCH" == "arm64" ]]; then
    ARCH="aarch64"
  fi

  local WASM_BINDGEN_VERSION
  WASM_BINDGEN_VERSION="$(get_wasm_bindgen_version)"
  curl -sSL "https://github.com/rustwasm/wasm-bindgen/releases/download/${WASM_BINDGEN_VERSION}/wasm-bindgen-${WASM_BINDGEN_VERSION}-${ARCH}-apple-darwin.tar.gz" |
    sudo tar -xzvf - -C /usr/local/bin --strip-components=1
}

install_safari() {
  # Safari ships with the OS and is preinstalled on the runner image;
  # we just need to enable the automation via the preinstalled safaridriver.
  sudo safaridriver --enable

  # Set SAFARIDRIVER env var.
  printf "SAFARIDRIVER=/usr/bin/safaridriver\n" >>"$GITHUB_ENV"
}

# Install the wasm test environment when a wasm test browser is requested.
case "${WASM_TEST_BROWSER:-}" in
safari)
  install_wasm_bindgen_test_runner
  install_safari
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

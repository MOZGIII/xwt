#!/bin/bash
# Shared helpers for setting up the wasm test environment.
# This file is sourced by the per-platform build env scripts.

get_wasm_bindgen_version() {
  cargo metadata \
    --format-version 1 \
    --manifest-path crates/xwt-web/Cargo.toml \
    --filter-platform wasm32-unknown-unknown \
    --quiet |
    jq -r \
      '.packages[] | select(.name == "wasm-bindgen").version'
}

# Print the wasm-bindgen release artifact target triple for the current system.
wasm_bindgen_target() {
  local ARCH
  ARCH="$(uname -m)"
  if [[ "$ARCH" == "arm64" ]]; then
    ARCH="aarch64"
  fi

  case "$(uname -s)" in
  Linux)
    printf "%s-unknown-linux-musl" "$ARCH"
    ;;
  Darwin)
    printf "%s-apple-darwin" "$ARCH"
    ;;
  *)
    printf "Unsupported system: %s\n" "$(uname -s)" >&2
    return 1
    ;;
  esac
}

install_wasm_bindgen_test_runner() {
  local TARGET
  TARGET="$(wasm_bindgen_target)"

  local WASM_BINDGEN_VERSION
  WASM_BINDGEN_VERSION="$(get_wasm_bindgen_version)"
  curl -sSL "https://github.com/rustwasm/wasm-bindgen/releases/download/${WASM_BINDGEN_VERSION}/wasm-bindgen-${WASM_BINDGEN_VERSION}-${TARGET}.tar.gz" |
    sudo tar -xzvf - -C /usr/local/bin --strip-components=1
}

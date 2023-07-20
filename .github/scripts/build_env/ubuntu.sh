#!/bin/bash
set -euo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

# Install wasm-bindgen-test-runner.
curl -sSL "https://github.com/rustwasm/wasm-bindgen/releases/download/0.2.87/wasm-bindgen-0.2.87-x86_64-unknown-linux-musl.tar.gz" |
  sudo tar -xzvf - -C /usr/local/bin --strip-components=1

source "${SCRIPT_DIR}/common.sh"

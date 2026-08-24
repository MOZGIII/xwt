#!/bin/bash
set -euo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

source "${SCRIPT_DIR}/lib/wasm_test.sh"

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

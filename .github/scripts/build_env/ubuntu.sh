#!/bin/bash
set -euo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

# Install wasm-bindgen-test-runner.
curl -sSL "https://github.com/rustwasm/wasm-bindgen/releases/download/0.2.87/wasm-bindgen-0.2.87-x86_64-unknown-linux-musl.tar.gz" |
  sudo tar -xzvf - -C /usr/local/bin --strip-components=1

# Install chromedriver.
npx @puppeteer/browsers install chromedriver@118.0.5993.32

# Set CHROMEDRIVER env var.
CHROMEDRIVERS=(chromedriver/*/chromedriver-*/chromedriver)
printf "CHROMEDRIVER=%s\n" "${CHROMEDRIVERS[0]}" >>"$GITHUB_ENV"

source "${SCRIPT_DIR}/common.sh"

#!/bin/bash
set -euo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

# Install wasm-bindgen-test-runner.
curl -sSL "https://github.com/rustwasm/wasm-bindgen/releases/download/0.2.88/wasm-bindgen-0.2.88-x86_64-unknown-linux-musl.tar.gz" |
  sudo tar -xzvf - -C /usr/local/bin --strip-components=1

# Install chromedriver and chrome.
CHROME_VERSION="119.0.6045.105"
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

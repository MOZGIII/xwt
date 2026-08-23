export type Mode = {
  name: string;
  cargoCommand: string;
  cargoArgs: string;
  cargoCacheKey: string;
  platformIndependent?: true;
  needsEchoServer?: true;
  env?: Record<string, string>;
};

export type Modes = Record<string, Mode>;

export type WasmTestBrowser = {
  name: string;
  env: Record<string, string>;
};

// The browsers (and their versions) to run the wasm tests in.
// The build env script interprets the env vars and installs the browser.
export const wasmTestBrowsers = {
  chrome_141: {
    name: "Chrome 141.0.7376.0",
    env: {
      WASM_TEST_BROWSER: "chrome",
      WASM_TEST_BROWSER_VERSION: "141.0.7376.0",
    },
  },
  chrome_154: {
    name: "Chrome 154.0.8016.0",
    env: {
      WASM_TEST_BROWSER: "chrome",
      WASM_TEST_BROWSER_VERSION: "154.0.8016.0",
    },
  },
  firefox_154: {
    name: "Firefox 154.0",
    env: {
      WASM_TEST_BROWSER: "firefox",
      WASM_TEST_BROWSER_VERSION: "154.0",
      WASM_TEST_GECKODRIVER_VERSION: "0.37.1",
    },
  },
} satisfies Record<string, WasmTestBrowser>;

// The `cargo-hack` version to install for the feature permutation checks.
// The build env script interprets the env var and installs the tool.
const cargoHackEnv = {
  INSTALL_CARGO_HACK_VERSION: "0.6.45",
};

export type FeaturePowersetExceptionGroup = {
  name: string;
  crates: string[];
  extraArgs: string;
};

// The exceptions to the feature powerset check.
//
// Some crates cannot be checked with the plain feature powerset invocation;
// each entry here defines a group of such crates, together with the extra
// `cargo hack` arguments that make the check work for them.
//
// The crates listed here are excluded from the main feature powerset run,
// and each group gets a dedicated generated mode instead.
export const featurePowersetExceptionGroups = {
  wtransport: {
    name: "wtransport crates",
    // The `wtransport` crate does not compile unless one of its TLS crypto
    // provider features is enabled, so keep one always enabled while checking
    // the feature permutations of the crates that depend on it.
    crates: ["xwt-wtransport", "xwt-test-server"],
    extraArgs: "--features wtransport/ring",
  },
} satisfies Record<string, FeaturePowersetExceptionGroup>;

// All the crates covered by the feature powerset exception groups; excluded
// from the main feature powerset run.
const featurePowersetExcludedCrates = Object.values(
  featurePowersetExceptionGroups as Record<string, FeaturePowersetExceptionGroup>
).flatMap((group) => group.crates);

// A feature powerset check mode for each of the exception groups.
const featuresExceptionModes = Object.fromEntries(
  Object.entries(
    featurePowersetExceptionGroups as Record<string, FeaturePowersetExceptionGroup>
  ).map(([key, group]) => [
    `features_${key}`,
    {
      name: `cargo hack clippy (feature powerset, ${group.name})`,
      cargoCommand: "hack",
      cargoArgs: [
        "clippy --locked --feature-powerset",
        ...group.crates.map((crate) => `--package ${crate}`),
        group.extraArgs,
        "-- -D warnings",
      ].join(" "),
      platformIndependent: true,
      cargoCacheKey: `features-${key}`,
      env: cargoHackEnv,
    } satisfies Mode,
  ])
) satisfies Modes;

// A wasm test mode for each of the browsers.
const testWasmModes = Object.fromEntries(
  Object.entries(wasmTestBrowsers as Record<string, WasmTestBrowser>).map(
    ([key, browser]) => [
      `test_wasm_${key}`,
      {
        name: `cargo test (wasm, ${browser.name})`,
        cargoCommand: "test",
        cargoArgs: "--locked --workspace --target wasm32-unknown-unknown",
        platformIndependent: true,
        cargoCacheKey: "test-wasm",
        needsEchoServer: true,
        env: browser.env,
      } satisfies Mode,
    ]
  )
) satisfies Modes;

export const code = {
  clippy: {
    name: "cargo clippy",
    cargoCommand: "clippy",
    cargoArgs: "--locked --workspace --all-targets -- -D warnings",
    cargoCacheKey: "clippy",
  },
  clippy_wasm: {
    name: "cargo clippy (wasm)",
    cargoCommand: "clippy",
    cargoArgs:
      "--locked --workspace --target wasm32-unknown-unknown --all-targets -- -D warnings",
    platformIndependent: true,
    cargoCacheKey: "clippy-wasm",
  },
  test: {
    name: "cargo test",
    cargoCommand: "test",
    cargoArgs: "--locked --workspace",
    cargoCacheKey: "test",
    needsEchoServer: true,
  },
  ...testWasmModes,
  build: {
    name: "cargo build",
    cargoCommand: "build",
    cargoArgs: "--locked --workspace",
    cargoCacheKey: "build",
  },
  build_wasm: {
    name: "cargo build (wasm)",
    cargoCommand: "build",
    cargoArgs: "--locked --workspace --target wasm32-unknown-unknown",
    platformIndependent: true,
    cargoCacheKey: "build-wasm",
  },
  features: {
    name: "cargo hack clippy (feature powerset)",
    cargoCommand: "hack",
    cargoArgs: [
      "clippy --locked --workspace --feature-powerset",
      ...featurePowersetExcludedCrates.map((crate) => `--exclude ${crate}`),
      "-- -D warnings",
    ].join(" "),
    platformIndependent: true,
    cargoCacheKey: "features",
    env: cargoHackEnv,
  },
  ...featuresExceptionModes,
  features_wasm: {
    name: "cargo hack clippy (feature powerset, wasm)",
    cargoCommand: "hack",
    cargoArgs:
      "clippy --locked --workspace --feature-powerset --target wasm32-unknown-unknown -- -D warnings",
    platformIndependent: true,
    cargoCacheKey: "features-wasm",
    env: cargoHackEnv,
  },
  fmt: {
    name: "cargo fmt",
    cargoCommand: "fmt",
    cargoArgs: "-- --check",
    platformIndependent: true,
    cargoCacheKey: "code",
  },
  docs: {
    name: "cargo doc",
    cargoCommand: "doc",
    cargoArgs: "--locked --workspace --document-private-items",
    platformIndependent: true,
    cargoCacheKey: "doc",
  },
  docs_wasm: {
    name: "cargo doc (wasm)",
    cargoCommand: "doc",
    cargoArgs:
      "--locked --workspace --target wasm32-unknown-unknown --document-private-items",
    platformIndependent: true,
    cargoCacheKey: "doc-wasm",
  },
} satisfies Modes;

export const build = {
  build: {
    name: "cargo build",
    cargoCommand: "build",
    cargoArgs: "--locked --workspace --release",
    cargoCacheKey: "release-build",
  },
} satisfies Modes;

// An utility to apply common build script paths.
const buildEnvScriptPath = (script) => `.github/scripts/build_env/${script}`;

// All the platforms that we support, and their respective settings.
const allPlatforms = {
  ubuntu2204: {
    name: "Ubuntu 22.04",
    os: "ubuntu-22.04",
    buildEnvScript: buildEnvScriptPath("ubuntu.sh"),
    isOnSelfHostedRunner: false,
    essential: true,
    env: {},
    cacheKey: "ubuntu2204-amd64",
    artifactMarker: "ubuntu2204",
    isBroken: false,
  },
  windows: {
    name: "Windows",
    os: "windows-latest",
    buildEnvScript: buildEnvScriptPath("windows.sh"),
    isOnSelfHostedRunner: false,
    essential: true,
    env: {
      CARGO_INCREMENTAL: "0",
    },
    cacheKey: "windows-amd64",
    artifactMarker: null,
    isBroken: false,
  },
  macos: {
    name: "macOS (amd64)",
    os: "macos-latest",
    buildEnvScript: buildEnvScriptPath("macos.sh"),
    isOnSelfHostedRunner: false,
    essential: true,
    env: {},
    cacheKey: "macos-amd64",
    artifactMarker: null,
    isBroken: false,
  },
};

// A platform for running things that are platform-independent.
const corePlatform = allPlatforms.ubuntu2204;

const codeModes = {
  clippy: {
    name: "clippy",
    cargoCommand: "clippy",
    cargoArgs: "--workspace --all-targets -- -D warnings",
    cargoCacheKey: "clippy",
  },
  clippy_wasm: {
    name: "clippy (wasm)",
    cargoCommand: "clippy",
    cargoArgs:
      "--workspace --target wasm32-unknown-unknown --all-targets -- -D warnings",
    platformIndependent: true,
    cargoCacheKey: "clippy-wasm",
  },
  test: {
    name: "test",
    cargoCommand: "test",
    cargoArgs: "--workspace",
    cargoCacheKey: "test",
    needsEchoServer: true,
  },
  test_wasm: {
    name: "test (wasm)",
    cargoCommand: "test",
    cargoArgs: "--workspace --target wasm32-unknown-unknown",
    platformIndependent: true,
    cargoCacheKey: "test-wasm",
    needsEchoServer: true,
  },
  build: {
    name: "build",
    cargoCommand: "build",
    cargoArgs: "--workspace",
    cargoCacheKey: "build",
  },
  build_wasm: {
    name: "build (wasm)",
    cargoCommand: "build",
    cargoArgs: "--workspace --target wasm32-unknown-unknown",
    platformIndependent: true,
    cargoCacheKey: "build-wasm",
  },
  fmt: {
    name: "fmt",
    cargoCommand: "fmt",
    cargoArgs: "-- --check",
    platformIndependent: true,
    cargoCacheKey: "code",
  },
  docs: {
    name: "doc",
    cargoCommand: "doc",
    cargoArgs: "--workspace --document-private-items",
    platformIndependent: true,
    cargoCacheKey: "doc",
  },
  docs_wasm: {
    name: "doc (wasm)",
    cargoCommand: "doc",
    cargoArgs:
      "--workspace --target wasm32-unknown-unknown --document-private-items",
    platformIndependent: true,
    cargoCacheKey: "doc",
  },
};

const buildModes = {};

const code = () => {
  // Compute the effective list of platforms to use.
  const effectivePlatforms = Object.values(allPlatforms).filter(
    (platform) => !platform.isBroken && platform.essential
  );

  // Compute the effective list of modes that should run for each of the platforms.
  const effectiveModes = Object.values(codeModes).filter(
    (mode) => !mode.platformIndependent
  );

  // Compute the effective list of modes that are platform indepedent and only
  // have to be run once.
  const effectiveIndepModes = Object.values(codeModes).filter(
    (mode) => mode.platformIndependent
  );

  // Compute the individual mixins for indep modes.
  const effectiveIncludes = effectiveIndepModes.map((mode) => ({
    // Run the platform independent tests on the core platform.
    platform: corePlatform,
    mode,
  }));

  // Prepare the effective matrix.
  const matrix = provideMatrix(
    {
      platform: effectivePlatforms,
      mode: effectiveModes,
    },
    effectiveIncludes
  );

  // Print the matrix, useful for local debugging.
  logMatrix(matrix);

  // Export the matrix so it's available to the Github Actions script.
  return matrix;
};

const build = () => {
  // Compute the effective list of platforms to use.
  const effectivePlatforms = Object.values(allPlatforms).filter(
    (platform) => !platform.isBroken
  );

  // Compute the effective list of modes that should run for each of the platforms.
  const effectiveModes = Object.values(buildModes);

  // Prepare the effective matrix.
  const matrix = provideMatrix(
    {
      platform: effectivePlatforms,
      mode: effectiveModes,
    },
    []
  );

  // Print the matrix, useful for local debugging.
  logMatrix(matrix);

  // Export the matrix so it's available to the Github Actions script.
  return matrix;
};

const evalMatrix = (dimensions, includes) => {
  const evalNext = (allVariants, key, values) =>
    allVariants.flatMap((variant) =>
      values.map((value) => ({ ...variant, [key]: value }))
    );
  const dimensionKeys = Object.keys(dimensions);
  const evaluated = dimensionKeys.reduce(
    (allVariants, dimensionKey) =>
      evalNext(allVariants, dimensionKey, dimensions[dimensionKey]),
    [{}]
  );
  return [...evaluated, ...includes];
};

const provideMatrix = (dimensions, includes) => ({
  plan: evalMatrix(dimensions, includes),
});

const logMatrix = (matrix) => console.log(JSON.stringify(matrix, null, "  "));

module.exports = {
  code,
  build,
};

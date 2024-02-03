export type Mode = {
  name: string;
  cargoCommand: string;
  cargoArgs: string;
  cargoCacheKey: string;
  platformIndependent?: true;
  needsEchoServer?: true;
};

export type Modes = Record<string, Mode>;

export const code = {
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
    cargoCacheKey: "doc-wasm",
  },
} satisfies Modes;

export const build = {
  build: {
    name: "build",
    cargoCommand: "build",
    cargoArgs: "--workspace --release",
    cargoCacheKey: "release-build",
  },
} satisfies Modes;

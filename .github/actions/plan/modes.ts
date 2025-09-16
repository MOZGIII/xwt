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
  test_wasm: {
    name: "cargo test (wasm)",
    cargoCommand: "test",
    cargoArgs: "--locked --workspace --target wasm32-unknown-unknown",
    platformIndependent: true,
    cargoCacheKey: "test-wasm",
    needsEchoServer: true,
  },
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

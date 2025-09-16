export type RunnerOS =
  | "ubuntu-24.04"
  | "ubuntu-24.04-arm"
  | "macos-15" // arm
  | "macos-15-large" // intel
  | "windows-latest";

export type Platform = {
  name: string;
  os: RunnerOS;
  buildEnvScript: string;
  essential: boolean;
  env: Record<string, string>;
  cacheKey: string;
  artifactMarker: string | null;
  isBroken: boolean;
};

export type Platforms = Record<string, Platform>;

// An utility to apply common build script paths.
const buildEnvScriptPath = (script: string) =>
  `.github/scripts/build_env/${script}`;

// All the platforms that we support, and their respective settings.
export const all = {
  ubuntu2404_amd64: {
    name: "Ubuntu 24.04 (amd64)",
    os: "ubuntu-24.04",
    buildEnvScript: buildEnvScriptPath("ubuntu.sh"),
    essential: true,
    env: {},
    cacheKey: "ubuntu2404-amd64",
    artifactMarker: "amd64-ubuntu2404",
    isBroken: false,
  },
  ubuntu2404_aarch64: {
    name: "Ubuntu 24.04 (aarch64)",
    os: "ubuntu-24.04-arm",
    buildEnvScript: buildEnvScriptPath("ubuntu.sh"),
    essential: false,
    env: {},
    cacheKey: "ubuntu2404-aarch64",
    artifactMarker: "aarch64-ubuntu2404",
    isBroken: false,
  },
  windows_amd64: {
    name: "Windows (amd64)",
    os: "windows-latest",
    buildEnvScript: buildEnvScriptPath("windows.sh"),
    essential: true,
    env: {},
    cacheKey: "windows-amd64",
    artifactMarker: "amd64",
    isBroken: false,
  },
  macos15_amd64: {
    name: "macOS 15 (amd64)",
    os: "macos-15-large",
    buildEnvScript: buildEnvScriptPath("macos.sh"),
    essential: false,
    env: {},
    cacheKey: "macos15-amd64",
    artifactMarker: "amd64",
    isBroken: true, // needs payment, because it's a `-large` runner
  },
  macos15_aarch64: {
    name: "macOS 15 (aarch64)",
    os: "macos-15",
    buildEnvScript: buildEnvScriptPath("macos.sh"),
    essential: true,
    env: {},
    cacheKey: "macos15-aarch64",
    artifactMarker: "aarch64",
    isBroken: false,
  },
} satisfies Platforms;

// A platform for running things that are platform-independent.
export const core = all.ubuntu2404_amd64 satisfies Platform;

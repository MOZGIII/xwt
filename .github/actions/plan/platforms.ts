export const selfHostedRunners = {} as const;

export type RunnerOS =
  | "ubuntu-22.04"
  | "windows-latest"
  | "macos-14"
  | (typeof selfHostedRunners)[keyof typeof selfHostedRunners];

export type Platform = {
  name: string;
  os: RunnerOS;
  isOnSelfHostedRunner: boolean;
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
    env: {},
    cacheKey: "windows-amd64",
    artifactMarker: null,
    isBroken: false,
  },
  macos: {
    name: "macOS",
    os: "macos-14",
    buildEnvScript: buildEnvScriptPath("macos.sh"),
    isOnSelfHostedRunner: false,
    essential: true,
    env: {},
    cacheKey: "macos-aarch64",
    artifactMarker: null,
    isBroken: false,
  },
} satisfies Platforms;

// A platform for running things that are platform-independent.
export const core = all.ubuntu2204 satisfies Platform;

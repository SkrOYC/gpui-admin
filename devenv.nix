{ pkgs, lib, ... }:

# Reproducible developer environment for gpui-admin.
#
# The committed `rust-toolchain.toml` is the source of truth for non-Nix
# contributors (their rustup honours it automatically). Inside this devenv shell
# Rust is provided by rust-overlay instead of rustup, and CI pins the version
# explicitly in ci.yml — all three are kept in agreement by hand on 1.95.0.
#
# GPUI compiles against a native graphics/windowing/font stack; on Linux those
# libraries are supplied here. On macOS they are system frameworks (Metal /
# CoreText / AppKit), so no packages are needed.

let
  gpuiLibs = with pkgs; [
    wayland
    libxkbcommon
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    xorg.libXext
    xorg.libxcb
    vulkan-loader
    fontconfig
    freetype
  ];
in
{
  # --- Rust toolchain, pinned to 1.95.0 via rust-overlay ---
  # `channel` is an enum (nixpkgs|stable|beta|nightly); the literal version goes
  # in `version`, which is honoured only when channel != "nixpkgs".
  languages.rust = {
    enable = true;
    channel = "stable";
    version = "1.95.0";
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  # --- Build + CLI tooling ---
  packages = with pkgs; [
    pkg-config        # *-sys crate discovery (freetype, fontconfig, wayland, xkbcommon)
    cmake             # aws-lc-rs / native build steps if a TLS backend is enabled later
    perl              # aws-lc-rs build dependency
    git
    gh                # GA-A003 / GA-A005 GitHub operations
    actionlint        # local linting of .github/workflows/ci.yml
  ] ++ lib.optionals pkgs.stdenv.isLinux gpuiLibs;

  # --- Link + discovery wiring for the GPUI native stack (Linux only) ---
  env = lib.mkIf pkgs.stdenv.isLinux {
    LD_LIBRARY_PATH = lib.makeLibraryPath gpuiLibs;
    PKG_CONFIG_PATH = lib.makeSearchPathOutput "dev" "lib/pkgconfig" gpuiLibs;
  };

  # --- Convenience: run the full local gate exactly as CI does ---
  # Mirrors ci.yml: RUSTFLAGS=-D warnings (workflow-wide there) and --locked on
  # every cargo step, so a green local run cannot turn red on push over a plain
  # rustc warning or a stale Cargo.lock.
  scripts.ci.exec = ''
    set -euo pipefail
    export RUSTFLAGS="-D warnings"
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets --locked -- -D warnings
    cargo build --workspace --locked
    cargo test --workspace --locked
  '';

  enterShell = ''
    echo "gpui-admin devenv — $(rustc --version 2>/dev/null || echo 'rust toolchain resolving…')"
  '';
}

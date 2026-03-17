#!/usr/bin/env bash
set -euo pipefail

OS="$(uname -s)"

if ! command -v cargo >/dev/null 2>&1; then
  echo "Error: cargo is not installed. Install Rust from https://rustup.rs/ first."
  exit 1
fi

if ! command -v git >/dev/null 2>&1; then
  echo "Error: git is not installed. Please install git first."
  exit 1
fi

case "$OS" in
  Linux|Darwin)
    echo "Detected OS: $OS"
    echo "==> Updating dependencies and building release"
    cargo update
    cargo build --release
    ;;

  MINGW*|MSYS*|CYGWIN*)
    echo "Detected Windows (Bash-like) environment: $OS"
    echo "==> Updating dependencies and building release"
    cargo update
    cargo build --release
    ;;

  *)
    echo "Unsupported OS: $OS"
    echo "Please use 'install.ps1' for Windows PowerShell, or run this on Linux/macOS bash."
    exit 1
    ;;
 esac

BIN_PATH="$(pwd)/target/release/http-req"
if [ ! -f "$BIN_PATH" ]; then
  echo "Error: Build failed or binary not found."
  exit 1
fi

echo "==> Installation complete. You can run the binary as:"
echo "    $BIN_PATH"

echo "Optional: link to PATH (macOS/Linux/Git Bash):"
echo "    sudo ln -sf $BIN_PATH /usr/local/bin/http-req"

echo "Windows PowerShell installer: install.ps1"

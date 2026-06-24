#!/usr/bin/env bash
# Above GoXLR - full release build script
# Usage: ./build-release.sh
set -e

ROOT="$(cd "$(dirname "$0")" && pwd)"

echo ""
echo "==> Above GoXLR Release Build"
echo ""

# 1. Build UI
echo "[1/2] Building UI..."
cd "$ROOT/ui"
[ ! -d node_modules ] && npm install
npm run build
echo "      UI built OK"

echo "      Copying UI to daemon/web-content..."
cp -r "$ROOT/ui/dist/." "$ROOT/daemon/web-content/"
echo "      Copied OK"

# 2. Build Rust
echo "[2/2] Building Rust (release)..."
cd "$ROOT"
cargo build --release
echo "      Rust built OK"

echo ""
echo "==> Output: $ROOT/target/release/"
echo "    goxlr-daemon"
echo "    goxlr-launcher"
echo ""
echo "==> To run: ./target/release/goxlr-daemon"
echo ""

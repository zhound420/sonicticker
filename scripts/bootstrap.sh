#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
echo "🚀 Bootstrapping Oscillator at ${ROOT_DIR}"

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "❌ Missing required command: $1" >&2
    exit 1
  fi
}

require_cmd cargo
require_cmd npm

echo "➡️  Installing Rust dependencies and building backend..."
pushd "${ROOT_DIR}/oscillator" >/dev/null
cargo fetch
cargo build
popd >/dev/null

echo "➡️  Installing frontend dependencies and producing production build..."
pushd "${ROOT_DIR}/frontend" >/dev/null
npm install
npm run build
popd >/dev/null

echo "✅ Bootstrap complete. Backend binary in oscillator/target/, frontend bundle in frontend/dist/"

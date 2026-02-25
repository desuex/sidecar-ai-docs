#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

SIDECAR_TMP_DIR=".sidecar-ci"
trap 'rm -rf "fixtures/ts-sample/${SIDECAR_TMP_DIR}"' EXIT

cargo build
cargo run -- --help
cargo run -- index --root fixtures/ts-sample --sidecar-dir "${SIDECAR_TMP_DIR}" --json
cargo run -- search CartService --root fixtures/ts-sample --sidecar-dir "${SIDECAR_TMP_DIR}" --json

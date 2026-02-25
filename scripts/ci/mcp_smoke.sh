#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

PROJECT_ROOT="${PROJECT_ROOT:-fixtures/ts-sample}"
if [[ "$PROJECT_ROOT" = /* ]]; then
  TARGET_ROOT="$PROJECT_ROOT"
else
  TARGET_ROOT="${ROOT_DIR}/${PROJECT_ROOT}"
fi

SIDECAR_TMP_DIR="${SIDECAR_TMP_DIR:-.sidecar-mcp-ci}"
SMOKE_QUERY="${SMOKE_QUERY:-CartService}"

trap 'rm -rf "${TARGET_ROOT}/${SIDECAR_TMP_DIR}"' EXIT
rm -rf "${TARGET_ROOT}/${SIDECAR_TMP_DIR}"

cargo run --quiet --bin sidecar -- --root "$TARGET_ROOT" --sidecar-dir "$SIDECAR_TMP_DIR" index --json >/dev/null

responses="$(
  printf '%s\n' \
    '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"mcp-smoke","version":"0.1.0"}}}' \
    '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' \
    "{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"tools/call\",\"params\":{\"name\":\"search_symbols\",\"arguments\":{\"query\":\"${SMOKE_QUERY}\",\"limit\":1}}}" \
  | cargo run --quiet --bin sidecar -- --root "$TARGET_ROOT" --sidecar-dir "$SIDECAR_TMP_DIR" mcp --log-level error
)"

init_line="$(echo "$responses" | sed -n '1p')"
tools_line="$(echo "$responses" | sed -n '2p')"
call_line="$(echo "$responses" | sed -n '3p')"

if [[ -z "$call_line" ]]; then
  echo "MCP smoke failed: expected 3 response lines."
  echo "$responses"
  exit 1
fi

echo "$init_line" | rg -q '"id":1'
echo "$init_line" | rg -q '"protocolVersion":"2024-11-05"'
echo "$init_line" | rg -q '"name":"sidecar-mcp"'

echo "$tools_line" | rg -q '"id":2'
echo "$tools_line" | rg -q '"name":"search_symbols"'
echo "$tools_line" | rg -q '"name":"coverage_metrics"'
echo "$tools_line" | rg -q '"name":"detect_undocumented_symbols"'

echo "$call_line" | rg -q '"id":3'
echo "$call_line" | rg -q '"structuredContent"'
echo "$call_line" | rg -q "\"${SMOKE_QUERY}\""

echo "MCP smoke test passed."

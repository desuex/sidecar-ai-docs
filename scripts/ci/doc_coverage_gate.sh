#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

BASELINE_FILE="${BASELINE_FILE:-scripts/ci/doc_coverage_baseline.env}"
SIDECAR_TMP_DIR="${SIDECAR_TMP_DIR:-.sidecar-ci-doccov}"

if [[ ! -f "$BASELINE_FILE" ]]; then
  echo "Missing baseline file: $BASELINE_FILE"
  exit 1
fi

# shellcheck disable=SC1090
source "$BASELINE_FILE"

: "${SCAN_LIMIT:?SCAN_LIMIT is required in baseline}"
: "${MIN_COVERAGE_PCT:?MIN_COVERAGE_PCT is required in baseline}"
: "${MAX_UNDOCUMENTED_PUBLIC:?MAX_UNDOCUMENTED_PUBLIC is required in baseline}"
: "${REQUIRE_SCAN_COMPLETE:=true}"

trap 'rm -rf "$SIDECAR_TMP_DIR"' EXIT
rm -rf "$SIDECAR_TMP_DIR"

cargo run --quiet --bin sidecar -- --root "$ROOT_DIR" --sidecar-dir "$SIDECAR_TMP_DIR" index --json >/dev/null

coverage_request="{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"coverage_metrics\",\"params\":{\"public_only\":true,\"scan_limit\":${SCAN_LIMIT}}}"
coverage_response="$(printf '%s\n' "$coverage_request" | cargo run --quiet --bin sidecar -- --root "$ROOT_DIR" --sidecar-dir "$SIDECAR_TMP_DIR" mcp --log-level error)"

undoc_request="{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"detect_undocumented_symbols\",\"params\":{\"public_only\":true,\"scan_limit\":${SCAN_LIMIT},\"limit\":1,\"offset\":0}}"
undoc_response="$(printf '%s\n' "$undoc_request" | cargo run --quiet --bin sidecar -- --root "$ROOT_DIR" --sidecar-dir "$SIDECAR_TMP_DIR" mcp --log-level error)"

coverage_pct="$(echo "$coverage_response" | sed -nE 's/.*"coverage_pct":([0-9.]+).*/\1/p')"
scan_complete="$(echo "$coverage_response" | sed -nE 's/.*"scan_complete":(true|false).*/\1/p')"
undocumented_total="$(echo "$undoc_response" | sed -nE 's/.*"undocumented_total":([0-9]+).*/\1/p')"

if [[ -z "$coverage_pct" || -z "$scan_complete" || -z "$undocumented_total" ]]; then
  echo "Failed to parse MCP coverage responses."
  echo "coverage_response=$coverage_response"
  echo "undoc_response=$undoc_response"
  exit 1
fi

echo "Documentation coverage: ${coverage_pct}%"
echo "Undocumented public symbols: ${undocumented_total}"
echo "Baseline minimum coverage: ${MIN_COVERAGE_PCT}%"
echo "Baseline max undocumented public symbols: ${MAX_UNDOCUMENTED_PUBLIC}"

if [[ "$REQUIRE_SCAN_COMPLETE" == "true" && "$scan_complete" != "true" ]]; then
  echo "Doc coverage gate failed: scan was incomplete (scan_complete=${scan_complete})."
  exit 1
fi

if ! awk -v cov="$coverage_pct" -v min="$MIN_COVERAGE_PCT" 'BEGIN { exit !(cov + 0 >= min + 0) }'; then
  echo "Doc coverage gate failed: coverage ${coverage_pct}% is below baseline ${MIN_COVERAGE_PCT}%."
  exit 1
fi

if ! awk -v undoc="$undocumented_total" -v max="$MAX_UNDOCUMENTED_PUBLIC" 'BEGIN { exit !(undoc + 0 <= max + 0) }'; then
  echo "Doc coverage gate failed: undocumented public symbols ${undocumented_total} exceeds baseline ${MAX_UNDOCUMENTED_PUBLIC}."
  exit 1
fi

echo "Doc coverage gate passed."

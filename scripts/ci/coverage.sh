#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

MIN_COVERAGE="${MIN_COVERAGE:-90}"
LCOV_FILE="${LCOV_FILE:-lcov.info}"

cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --all-features --lcov --output-path "$LCOV_FILE"

# Compute line coverage from lcov for a deterministic threshold gate.
coverage_pct="$(
  awk -F'[:,]' '
    BEGIN { total = 0; covered = 0; }
    /^DA:/ { total++; if ($3 > 0) covered++; }
    END {
      if (total == 0) {
        print "0.00";
      } else {
        printf "%.2f", (covered / total) * 100;
      }
    }
  ' "$LCOV_FILE"
)"

echo "Computed line coverage: ${coverage_pct}%"
echo "Required minimum coverage: ${MIN_COVERAGE}%"

if ! awk -v cov="$coverage_pct" -v min="$MIN_COVERAGE" 'BEGIN { exit !(cov + 0 >= min + 0) }'; then
  echo "Coverage gate failed: ${coverage_pct}% is below ${MIN_COVERAGE}%"
  exit 1
fi

echo "Coverage gate passed."

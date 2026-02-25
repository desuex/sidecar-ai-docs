#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SRC_DIR="${ROOT_DIR}/docs-sidecar"
OUT_DIR="${ROOT_DIR}/docs/generated"

mkdir -p "${OUT_DIR}"

cat > "${OUT_DIR}/README.md" <<'EOF'
# Generated Sidecar Docs

This directory contains generated documentation intended for MkDocs/RTD publishing.

Current stage:

- Bootstrap export script
- Deterministic copy-through from `docs-sidecar/**/*.md`

Do not hand-edit generated files.
EOF

if [[ ! -d "${SRC_DIR}" ]]; then
  echo "No docs-sidecar directory found at ${SRC_DIR}; export skipped."
  exit 0
fi

mkdir -p "${OUT_DIR}/symbols"

# Clear previous copied symbol files to keep output deterministic.
find "${OUT_DIR}/symbols" -type f -name '*.md' -delete

count=0
while IFS= read -r source_file; do
  [[ -z "${source_file}" ]] && continue
  base_name="$(basename "${source_file}")"
  target_file="${OUT_DIR}/symbols/${base_name}"
  cp "${source_file}" "${target_file}"
  count=$((count + 1))
done < <(find "${SRC_DIR}" -type f -name '*.md' | LC_ALL=C sort)

echo "Exported ${count} sidecar markdown file(s) to ${OUT_DIR}/symbols"

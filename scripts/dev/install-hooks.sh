#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
HOOKS_DIR="${ROOT_DIR}/.git/hooks"

if [[ ! -d "${ROOT_DIR}/.git" ]]; then
  echo "Not a git repository: ${ROOT_DIR}" >&2
  exit 1
fi

mkdir -p "${HOOKS_DIR}"

cat > "${HOOKS_DIR}/pre-commit" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

cargo fmt --all
EOF

cat > "${HOOKS_DIR}/pre-push" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

cargo test --all --all-features
EOF

chmod +x "${HOOKS_DIR}/pre-commit" "${HOOKS_DIR}/pre-push"
echo "Installed hooks: pre-commit (fmt), pre-push (test)"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SRC_DIR="${ROOT_DIR}/docs-sidecar"
OUT_DIR="${SIDECAR_EXPORT_OUT_DIR:-${ROOT_DIR}/docs/generated}"
if [[ "${OUT_DIR}" != /* ]]; then
  OUT_DIR="${ROOT_DIR}/${OUT_DIR}"
fi
SYMBOLS_DIR="${OUT_DIR}/symbols"
REPORTS_DIR="${OUT_DIR}/reports"
MANIFEST_PATH="${OUT_DIR}/_manifest.json"
UNRESOLVED_REPORT_PATH="${OUT_DIR}/reports/unresolved-anchors.md"
ANCHOR_SEP=$'\x1f'
INDEX_DB_PATH="${SIDECAR_INDEX_DB_PATH:-${ROOT_DIR}/.sidecar/index.sqlite}"
INDEX_DB_WAL_PATH="${INDEX_DB_PATH}-wal"
INDEX_DB_SHM_PATH="${INDEX_DB_PATH}-shm"

to_repo_relative() {
  local path="$1"
  if [[ "${path}" == "${ROOT_DIR}" ]]; then
    echo "."
    return
  fi
  if [[ "${path}" == "${ROOT_DIR}/"* ]]; then
    echo "${path#${ROOT_DIR}/}"
    return
  fi
  echo "${path}"
}

INDEX_DB_PATH_DISPLAY="$(to_repo_relative "${INDEX_DB_PATH}")"
OUT_DIR_REL="$(to_repo_relative "${OUT_DIR}")"
UNRESOLVED_REPORT_REL="${OUT_DIR_REL}/reports/unresolved-anchors.md"

detect_source_blob_base() {
  local remote_url
  remote_url="$(git -C "${ROOT_DIR}" config --get remote.origin.url 2>/dev/null || true)"
  if [[ -z "${remote_url}" ]]; then
    echo "https://github.com/desuex/sidecar-ai-docs/blob/main"
    return
  fi

  if [[ "${remote_url}" == git@github.com:* ]]; then
    remote_url="${remote_url#git@github.com:}"
    remote_url="${remote_url%.git}"
    echo "https://github.com/${remote_url}/blob/main"
    return
  fi

  if [[ "${remote_url}" == https://github.com/* ]]; then
    remote_url="${remote_url#https://github.com/}"
    remote_url="${remote_url%.git}"
    echo "https://github.com/${remote_url}/blob/main"
    return
  fi

  echo "https://github.com/desuex/sidecar-ai-docs/blob/main"
}

SOURCE_BLOB_BASE="${SIDECAR_DOCS_SOURCE_BLOB_BASE:-$(detect_source_blob_base)}"

mkdir -p "${OUT_DIR}"

cat > "${OUT_DIR}/README.md" <<'EOF'
# Generated Sidecar Docs

This directory contains generated documentation intended for MkDocs/RTD publishing.

This output is generated from `docs-sidecar/` by `scripts/docs/export-sidecar-to-mkdocs.sh`.
Pages are deterministic and include:

- Parsed `doc_uid` and title
- Extracted summary (`## Overview` section or first paragraph)
- Anchor list
- Anchor validation report (`reports/unresolved-anchors.md`)
- Source links back to `docs-sidecar/*`

Do not hand-edit generated files.
EOF

mkdir -p "${SYMBOLS_DIR}"
mkdir -p "${REPORTS_DIR}"

# Clear previous copied symbol files to keep output deterministic.
find "${SYMBOLS_DIR}" -type f -name '*.md' -delete

json_escape() {
  printf '%s' "$1" | sed -e 's/\\/\\\\/g' -e 's/"/\\"/g'
}

yaml_value() {
  local key="$1"
  local front_matter_file="$2"
  awk -v key="${key}" '
    function trim(s) {
      sub(/^[[:space:]]+/, "", s)
      sub(/[[:space:]]+$/, "", s)
      return s
    }
    {
      if ($0 ~ "^[[:space:]]*" key ":[[:space:]]*") {
        value = $0
        sub("^[[:space:]]*" key ":[[:space:]]*", "", value)
        value = trim(value)
        gsub(/^"/, "", value)
        gsub(/"$/, "", value)
        print value
        exit
      }
    }
  ' "${front_matter_file}"
}

extract_front_matter_and_body() {
  local source_file="$1"
  local front_matter_file="$2"
  local body_file="$3"
  : > "${front_matter_file}"
  : > "${body_file}"

  awk -v front_matter_file="${front_matter_file}" -v body_file="${body_file}" '
    BEGIN {
      in_front_matter = 0
      saw_open = 0
      saw_close = 0
    }

    NR == 1 {
      if ($0 ~ /^---[[:space:]]*$/) {
        in_front_matter = 1
        saw_open = 1
        next
      }
      exit 2
    }

    {
      if (in_front_matter) {
        if ($0 ~ /^---[[:space:]]*$/) {
          in_front_matter = 0
          saw_close = 1
          next
        }
        print >> front_matter_file
      } else {
        print >> body_file
      }
    }

    END {
      if (!saw_open || !saw_close) {
        exit 2
      }
    }
  ' "${source_file}"
}

extract_summary() {
  local body_file="$1"
  awk '
    function trim(s) {
      sub(/^[[:space:]]+/, "", s)
      sub(/[[:space:]]+$/, "", s)
      return s
    }

    {
      lines[NR] = $0
    }

    END {
      # Prefer `## Overview` section.
      for (i = 1; i <= NR; i++) {
        if (trim(lines[i]) == "## Overview") {
          summary = ""
          for (j = i + 1; j <= NR; j++) {
            if (trim(lines[j]) ~ /^##[[:space:]]+/) {
              break
            }
            if (summary == "") {
              summary = lines[j]
            } else {
              summary = summary "\n" lines[j]
            }
          }
          summary = trim(summary)
          if (summary != "") {
            print summary
            exit
          }
        }
      }

      # Fallback to first non-empty paragraph.
      summary = ""
      started = 0
      for (i = 1; i <= NR; i++) {
        current = trim(lines[i])
        if (!started) {
          if (current == "") {
            continue
          }
          started = 1
        }
        if (started && current == "") {
          break
        }
        if (summary == "") {
          summary = lines[i]
        } else {
          summary = summary "\n" lines[i]
        }
      }

      summary = trim(summary)
      if (summary != "") {
        print summary
      }
    }
  ' "${body_file}"
}

extract_anchors() {
  local front_matter_file="$1"
  local anchors_file="$2"
  local sep="$3"
  awk -v sep="${sep}" '
    function trim(s) {
      sub(/^[[:space:]]+/, "", s)
      sub(/[[:space:]]+$/, "", s)
      return s
    }
    function flush_anchor() {
      if (in_anchor) {
        print anchor_type sep symbol_uid sep fingerprint sep confidence
      }
    }
    {
      if ($0 ~ /^[[:space:]]*-[[:space:]]*anchor_type:[[:space:]]*/) {
        flush_anchor()
        in_anchor = 1
        anchor_type = $0
        sub(/^[[:space:]]*-[[:space:]]*anchor_type:[[:space:]]*/, "", anchor_type)
        anchor_type = trim(anchor_type)
        symbol_uid = ""
        fingerprint = ""
        confidence = ""
        next
      }
      if (!in_anchor) {
        next
      }
      if ($0 ~ /^[[:space:]]*symbol_uid:[[:space:]]*/) {
        symbol_uid = $0
        sub(/^[[:space:]]*symbol_uid:[[:space:]]*/, "", symbol_uid)
        symbol_uid = trim(symbol_uid)
      } else if ($0 ~ /^[[:space:]]*fingerprint:[[:space:]]*/) {
        fingerprint = $0
        sub(/^[[:space:]]*fingerprint:[[:space:]]*/, "", fingerprint)
        fingerprint = trim(fingerprint)
      } else if ($0 ~ /^[[:space:]]*confidence:[[:space:]]*/) {
        confidence = $0
        sub(/^[[:space:]]*confidence:[[:space:]]*/, "", confidence)
        confidence = trim(confidence)
      }
    }
    END {
      flush_anchor()
    }
  ' "${front_matter_file}" > "${anchors_file}"
}

write_manifest() {
  local rows_file="$1"
  local count
  count="$(wc -l < "${rows_file}" | tr -d ' ')"
  {
    echo "{"
    echo "  \"source_dir\": \"docs-sidecar\","
    echo "  \"output_dir\": \"${OUT_DIR_REL}/symbols\","
    echo "  \"count\": ${count},"
    echo "  \"pages\": ["

    local i=0
    while IFS=$'\t' read -r doc_uid source_rel output_rel _filename title source_url anchor_count unresolved_count; do
      [[ -z "${doc_uid}" ]] && continue
      i=$((i + 1))
      local comma=","
      if [[ "${i}" -eq "${count}" ]]; then
        comma=""
      fi
      printf '    {"doc_uid":"%s","title":"%s","source":"%s","source_url":"%s","output":"%s","anchor_count":%s,"unresolved_anchor_count":%s}%s\n' \
        "$(json_escape "${doc_uid}")" \
        "$(json_escape "${title}")" \
        "$(json_escape "${source_rel}")" \
        "$(json_escape "${source_url}")" \
        "$(json_escape "${output_rel}")" \
        "${anchor_count}" \
        "${unresolved_count}" \
        "${comma}"
    done < "${rows_file}"

    echo "  ],"
    printf '  "validation": {"mode":"%s","status":"%s","index_db":"%s","report":"%s","symbol_anchors_checked":%s,"symbol_anchors_resolved":%s,"symbol_anchors_unresolved":%s,"symbol_anchors_skipped":%s}\n' \
      "$(json_escape "${VALIDATION_MODE}")" \
      "$(json_escape "${VALIDATION_STATUS_TEXT}")" \
      "$(json_escape "${INDEX_DB_PATH_DISPLAY}")" \
      "${UNRESOLVED_REPORT_REL}" \
      "${SYMBOL_ANCHORS_CHECKED}" \
      "${RESOLVED_SYMBOL_ANCHORS}" \
      "${UNRESOLVED_SYMBOL_ANCHORS}" \
      "${SKIPPED_SYMBOL_ANCHORS}"
    echo "}"
  } > "${MANIFEST_PATH}"
}

write_unresolved_report() {
  local unresolved_rows_file="$1"
  {
    echo "# Unresolved Symbol Anchors Report"
    echo
    echo "- Validation mode: \`${VALIDATION_MODE}\`"
    echo "- Validation status: ${VALIDATION_STATUS_TEXT}"
    echo "- Index database: \`${INDEX_DB_PATH_DISPLAY}\`"
    echo "- Generated by: \`scripts/docs/export-sidecar-to-mkdocs.sh\`"
    echo
    echo "## Summary"
    echo
    echo "- Symbol anchors checked: ${SYMBOL_ANCHORS_CHECKED}"
    echo "- Symbol anchors resolved: ${RESOLVED_SYMBOL_ANCHORS}"
    echo "- Symbol anchors unresolved: ${UNRESOLVED_SYMBOL_ANCHORS}"
    echo "- Symbol anchors skipped: ${SKIPPED_SYMBOL_ANCHORS}"
    echo
    echo "## Results"
    echo

    if [[ "${VALIDATION_MODE}" != "active" ]]; then
      echo "Validation did not run in this environment."
      return
    fi

    if [[ ! -s "${unresolved_rows_file}" ]]; then
      echo "No unresolved symbol anchors detected."
      return
    fi

    echo "| Doc UID | Source Doc | Symbol UID | Reason |"
    echo "|---|---|---|---|"
    while IFS=$'\t' read -r doc_uid source_rel output_rel source_url symbol_uid reason; do
      output_page_rel="${output_rel#${OUT_DIR_REL}/}"
      printf '| [\`%s\`](../%s) | [\`%s\`](%s) | \`%s\` | %s |\n' \
        "${doc_uid}" \
        "${output_page_rel}" \
        "${source_rel}" \
        "${source_url}" \
        "${symbol_uid}" \
        "${reason}"
    done < "${unresolved_rows_file}"
  } > "${UNRESOLVED_REPORT_PATH}"
}

if [[ ! -d "${SRC_DIR}" ]]; then
  VALIDATION_MODE="skipped_missing_docs_sidecar"
  VALIDATION_STATUS_TEXT="docs-sidecar directory not found"
  SYMBOL_ANCHORS_CHECKED=0
  RESOLVED_SYMBOL_ANCHORS=0
  UNRESOLVED_SYMBOL_ANCHORS=0
  SKIPPED_SYMBOL_ANCHORS=0
  write_unresolved_report /dev/null
  write_manifest /dev/null
  echo "No docs-sidecar directory found at ${SRC_DIR}; export skipped."
  exit 0
fi

rows_file="$(mktemp "${TMPDIR:-/tmp}/sidecar-export-rows.XXXXXX")"
sorted_rows_file="$(mktemp "${TMPDIR:-/tmp}/sidecar-export-sorted.XXXXXX")"
unresolved_rows_file="$(mktemp "${TMPDIR:-/tmp}/sidecar-export-unresolved.XXXXXX")"
symbols_uids_file="$(mktemp "${TMPDIR:-/tmp}/sidecar-export-symbols.XXXXXX")"
front_matter_file="$(mktemp "${TMPDIR:-/tmp}/sidecar-export-front.XXXXXX")"
body_file="$(mktemp "${TMPDIR:-/tmp}/sidecar-export-body.XXXXXX")"
anchors_file="$(mktemp "${TMPDIR:-/tmp}/sidecar-export-anchors.XXXXXX")"
cleanup() {
  rm -f \
    "${rows_file}" \
    "${sorted_rows_file}" \
    "${unresolved_rows_file}" \
    "${symbols_uids_file}" \
    "${front_matter_file}" \
    "${body_file}" \
    "${anchors_file}"
}
trap cleanup EXIT

VALIDATION_MODE="active"
VALIDATION_STATUS_TEXT="symbol UID validation enabled"
INDEX_WAL_PREEXISTED=0
INDEX_SHM_PREEXISTED=0

if ! command -v sqlite3 >/dev/null 2>&1; then
  VALIDATION_MODE="skipped_no_sqlite3"
  VALIDATION_STATUS_TEXT="sqlite3 not found; symbol UID validation skipped"
elif [[ ! -f "${INDEX_DB_PATH}" ]]; then
  VALIDATION_MODE="skipped_missing_index_db"
  VALIDATION_STATUS_TEXT="index database not found at ${INDEX_DB_PATH_DISPLAY}"
else
  if [[ -f "${INDEX_DB_WAL_PATH}" ]]; then
    INDEX_WAL_PREEXISTED=1
  fi
  if [[ -f "${INDEX_DB_SHM_PATH}" ]]; then
    INDEX_SHM_PREEXISTED=1
  fi

  if ! sqlite3 -batch -noheader "${INDEX_DB_PATH}" "SELECT uid FROM symbols ORDER BY uid;" > "${symbols_uids_file}" 2>/dev/null; then
    VALIDATION_MODE="skipped_index_query_error"
    VALIDATION_STATUS_TEXT="failed to query symbols table from ${INDEX_DB_PATH_DISPLAY}"
  fi

  if [[ "${INDEX_WAL_PREEXISTED}" -eq 0 ]]; then
    rm -f "${INDEX_DB_WAL_PATH}"
  fi
  if [[ "${INDEX_SHM_PREEXISTED}" -eq 0 ]]; then
    rm -f "${INDEX_DB_SHM_PATH}"
  fi
fi

if [[ "${VALIDATION_MODE}" != "active" ]]; then
  : > "${symbols_uids_file}"
fi

SYMBOL_ANCHORS_CHECKED=0
RESOLVED_SYMBOL_ANCHORS=0
UNRESOLVED_SYMBOL_ANCHORS=0
SKIPPED_SYMBOL_ANCHORS=0

while IFS= read -r source_file; do
  [[ -z "${source_file}" ]] && continue

  rel_source="${source_file#${ROOT_DIR}/}"
  if ! extract_front_matter_and_body "${source_file}" "${front_matter_file}" "${body_file}"; then
    echo "Exporter error: invalid YAML front matter in ${rel_source}" >&2
    exit 1
  fi

  doc_uid="$(yaml_value "doc_uid" "${front_matter_file}")"
  if [[ -z "${doc_uid}" ]]; then
    echo "Exporter error: missing doc_uid in front matter: ${rel_source}" >&2
    exit 1
  fi

  title="$(yaml_value "title" "${front_matter_file}")"
  if [[ -z "${title}" ]]; then
    echo "Exporter error: missing title in front matter: ${rel_source}" >&2
    exit 1
  fi

  summary="$(extract_summary "${body_file}")"
  extract_anchors "${front_matter_file}" "${anchors_file}" "${ANCHOR_SEP}"
  if [[ -s "${anchors_file}" ]]; then
    LC_ALL=C sort -t "${ANCHOR_SEP}" -k1,1 -k2,2 -k3,3 -k4,4 "${anchors_file}" -o "${anchors_file}"
  fi

  filename="$(printf '%s' "${doc_uid}" | sed -E 's/[^A-Za-z0-9._-]+/-/g; s/^-+//; s/-+$//')"
  if [[ -z "${filename}" ]]; then
    echo "Exporter error: doc_uid produced empty filename: ${rel_source}" >&2
    exit 1
  fi
  filename="${filename}.md"

  output_rel="${OUT_DIR_REL}/symbols/${filename}"
  source_url="${SOURCE_BLOB_BASE}/${rel_source}"
  output_file="${SYMBOLS_DIR}/${filename}"
  doc_unresolved_count=0

  {
    echo "# ${title}"
    echo
    echo "- Doc UID: \`${doc_uid}\`"
    echo "- Source: [\`${rel_source}\`](${source_url})"
    echo "- Generated by: \`scripts/docs/export-sidecar-to-mkdocs.sh\`"
    echo
    echo "## Summary"
    echo
    if [[ -n "${summary}" ]]; then
      printf '%s\n' "${summary}"
    else
      echo "(no summary available)"
    fi
    echo
    echo "## Anchors"
    echo
    if [[ -s "${anchors_file}" ]]; then
      while IFS="${ANCHOR_SEP}" read -r anchor_type symbol_uid fingerprint confidence; do
        line="- type=\`${anchor_type}\`"
        if [[ -n "${symbol_uid}" ]]; then
          line="${line}, symbol_uid=\`${symbol_uid}\`"
        fi
        if [[ -n "${fingerprint}" ]]; then
          line="${line}, fingerprint=\`${fingerprint}\`"
        fi
        if [[ -n "${confidence}" ]]; then
          line="${line}, confidence=\`${confidence}\`"
        fi
        if [[ "${anchor_type}" == "symbol" ]]; then
          if [[ "${VALIDATION_MODE}" == "active" ]]; then
            SYMBOL_ANCHORS_CHECKED=$((SYMBOL_ANCHORS_CHECKED + 1))
            if [[ -z "${symbol_uid}" ]]; then
              UNRESOLVED_SYMBOL_ANCHORS=$((UNRESOLVED_SYMBOL_ANCHORS + 1))
              doc_unresolved_count=$((doc_unresolved_count + 1))
              symbol_uid_display="<missing>"
              reason="missing symbol_uid in anchor"
              status="invalid_missing_symbol_uid"
              printf '%s\t%s\t%s\t%s\t%s\t%s\n' \
                "${doc_uid}" \
                "${rel_source}" \
                "${output_rel}" \
                "${source_url}" \
                "${symbol_uid_display}" \
                "${reason}" >> "${unresolved_rows_file}"
            elif grep -F -x -q -- "${symbol_uid}" "${symbols_uids_file}"; then
              RESOLVED_SYMBOL_ANCHORS=$((RESOLVED_SYMBOL_ANCHORS + 1))
              status="resolved"
            else
              UNRESOLVED_SYMBOL_ANCHORS=$((UNRESOLVED_SYMBOL_ANCHORS + 1))
              doc_unresolved_count=$((doc_unresolved_count + 1))
              reason="symbol_uid not found in symbols table"
              status="unresolved_symbol_uid"
              printf '%s\t%s\t%s\t%s\t%s\t%s\n' \
                "${doc_uid}" \
                "${rel_source}" \
                "${output_rel}" \
                "${source_url}" \
                "${symbol_uid}" \
                "${reason}" >> "${unresolved_rows_file}"
            fi
          else
            SKIPPED_SYMBOL_ANCHORS=$((SKIPPED_SYMBOL_ANCHORS + 1))
            status="not_checked"
          fi
          line="${line}, status=\`${status}\`"
        fi
        echo "${line}"
      done < "${anchors_file}"
    else
      echo "- (no anchors)"
    fi
    echo
    echo "## Content"
    echo
    awk '
      BEGIN { started = 0 }
      {
        if (!started && $0 ~ /^[[:space:]]*$/) {
          next
        }
        started = 1
        print
      }
    ' "${body_file}"
  } > "${output_file}"

  safe_title="${title//$'\t'/ }"
  anchor_count="$(wc -l < "${anchors_file}" | tr -d ' ')"
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
    "${doc_uid}" \
    "${rel_source}" \
    "${output_rel}" \
    "${filename}" \
    "${safe_title}" \
    "${source_url}" \
    "${anchor_count}" \
    "${doc_unresolved_count}" >> "${rows_file}"
done < <(find "${SRC_DIR}" -type f -name '*.md' | LC_ALL=C sort)

duplicate_filenames="$(cut -f4 "${rows_file}" | LC_ALL=C sort | uniq -d)"
if [[ -n "${duplicate_filenames}" ]]; then
  echo "Exporter error: duplicate generated filenames detected:" >&2
  printf '%s\n' "${duplicate_filenames}" >&2
  exit 1
fi

if [[ -s "${unresolved_rows_file}" ]]; then
  LC_ALL=C sort -t $'\t' -k1,1 -k5,5 -k2,2 "${unresolved_rows_file}" -o "${unresolved_rows_file}"
fi

write_unresolved_report "${unresolved_rows_file}"

LC_ALL=C sort -t $'\t' -k1,1 -k2,2 "${rows_file}" > "${sorted_rows_file}"
write_manifest "${sorted_rows_file}"

count="$(wc -l < "${sorted_rows_file}" | tr -d ' ')"
echo "Exported ${count} sidecar markdown file(s) to ${SYMBOLS_DIR}"
echo "Wrote deterministic manifest: ${MANIFEST_PATH}"
echo "Wrote unresolved anchor report: ${UNRESOLVED_REPORT_PATH}"

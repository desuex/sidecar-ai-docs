# CLI Specification

---

## 1. Purpose

This document defines the command-line interface (CLI) for interacting with:

* The symbol index
* The documentation sidecar
* Anchors
* UID mappings
* MCP-equivalent tools
* Index lifecycle

The CLI must provide:

* Human-friendly commands
* Scriptable JSON output
* Deterministic behavior
* Parity with MCP tools
* Safe refactor workflows

CLI is the human-facing interface.

---

## 2. Design Principles

CLI must be:

* Deterministic
* Composable
* Scriptable
* JSON-first
* Safe by default
* Refactor-aware
* Fast
* Readable

CLI must not:

* Hide errors
* Perform silent rebinding
* Modify source files
* Auto-generate documentation content
* Depend on interactive UI

---

## 3. Command Structure

Canonical command prefix:

```text
sidecar <command> [options]
```

Commands grouped into:

1. Index commands
2. Query commands
3. Documentation commands
4. Anchor commands
5. Diagnostics commands
6. Migration commands

---

## 4. Index Commands

---

### 4.1 Initialize Index

```text
sidecar init
```

Creates:

* .sidecar directory
* index storage
* schema metadata

---

### 4.2 Full Rebuild

```text
sidecar index
```

Rebuild entire index from source.

Options:

```text
--force
--verbose
--lang <language>
```

---

### 4.3 Incremental Update

```text
sidecar update
```

Re-index changed files only.

Must compare file hashes.

---

### 4.4 Index Status

```text
sidecar status
```

Returns:

* Total symbols
* Total references
* Unresolved anchors
* Schema version
* Last indexed timestamp

---

## 5. Query Commands

---

### 5.1 Get Symbol

```text
sidecar symbol <uid>
```

Options:

```text
--json
--fields uid,name,signature
```

---

### 5.2 Find References

```text
sidecar refs <uid>
```

Options:

```text
--limit 50
--offset 0
--json
```

---

### 5.3 Search Symbols

```text
sidecar search-symbols "query"
```

Options:

```text
--kind method
--limit 20
--json
```

---

### 5.4 Impact Analysis

```text
sidecar impact <uid>
```

Options:

```text
--depth 1
--json
```

---

### 5.5 Get Documentation

```text
sidecar doc <doc_uid>
```

Options:

```text
--json
--raw
```

---

## 6. Documentation Commands

---

### 6.1 Create Documentation

```text
sidecar doc-create --symbol <uid> --title "Title"
```

Creates sidecar file with:

* Metadata header
* Anchor binding
* Empty markdown body

Must not auto-fill content.

---

### 6.2 Attach Documentation to Symbol

```text
sidecar doc-attach <doc_uid> --symbol <uid>
```

Adds anchor entry.

---

### 6.3 Detach Anchor

```text
sidecar doc-detach <doc_uid> --symbol <uid>
```

Removes anchor entry.

---

### 6.4 List Docs for Symbol

```text
sidecar doc-list <uid>
```

---

### 6.5 Search Docs

```text
sidecar doc-search "query"
```

Options:

```text
--limit 20
--json
```

---

## 7. Anchor Commands

---

### 7.1 Validate Anchors

```text
sidecar anchors-validate
```

Outputs:

* Unresolved anchors
* Low-confidence anchors
* Migration suggestions

---

### 7.2 Rebind Anchors

```text
sidecar anchors-rebind
```

Options:

```text
--threshold 0.85
--dry-run
```

Must not silently rebind without reporting.

---

### 7.3 Show Anchor History

```text
sidecar anchor-history <doc_uid>
```

Shows:

* Old UID
* New UID
* Similarity score
* Timestamp

---

## 8. Diagnostics Commands

---

### 8.1 List Unresolved Documentation

```text
sidecar unresolved
```

---

### 8.2 Coverage Report

```text
sidecar coverage
```

Outputs:

* % documented symbols
* Undocumented public symbols
* Concepts without symbols

---

### 8.3 Integrity Check

```text
sidecar check
```

Verifies:

* UID uniqueness
* Broken references
* Schema consistency
* Missing sidecar files

---

## 9. Migration Commands

---

### 9.1 Show UID Migrations

```text
sidecar uid-migrations
```

---

### 9.2 Export Migrations

```text
sidecar uid-migrations --json
```

---

### 9.3 Reset Index

```text
sidecar reset
```

Deletes index storage only.

Does not delete documentation.

---

## 10. JSON Output Mode

All commands must support:

```text
--json
```

JSON output must:

* Be deterministic
* Match MCP schema
* Avoid extra fields
* Support piping

Example:

```text
sidecar symbol sym:... --json | jq .
```

---

## 11. Non-Interactive Mode

CLI must:

* Avoid interactive prompts by default
* Require explicit flags for destructive actions
* Support scripting environments
* Exit with proper error codes

Exit codes:

* 0 → success
* 1 → general error
* 2 → validation error
* 3 → not found
* 4 → index unavailable

---

## 12. Determinism

CLI must guarantee:

* Same command → same output
* Stable ordering
* No random ordering
* No time-dependent behavior in output

---

## 13. Performance Requirements

CLI commands must:

* Return symbol lookup under 100ms
* Support large repositories
* Avoid full index scan unless necessary
* Respect pagination

---

## 14. Safety Rules

CLI must:

* Refuse destructive commands without confirmation flag
* Prevent accidental deletion of documentation
* Not modify source files
* Not auto-generate content
* Validate UID inputs

---

## 15. Integration with MCP

Every CLI command must map to:

* Equivalent MCP tool
* Same query core
* Same schema
* Same validation rules

CLI is thin wrapper over MCP core.

---

## 16. Extensibility

Future commands may include:

* doc-suggest
* doc-generate (AI-assisted)
* graph-export
* stats
* semantic-search

Extensions must not break deterministic core.

---

## 17. Non-Goals

CLI does not:

* Replace IDE integration
* Replace documentation site generator
* Execute source code
* Run tests
* Perform semantic inference beyond index

CLI is an index interaction tool.

---

## 18. Summary

The CLI provides:

* Deterministic control over index
* Refactor-safe documentation management
* Token-efficient AI parity
* Scriptable interface
* Audit-friendly operations

It is the developer’s gateway to structured knowledge.
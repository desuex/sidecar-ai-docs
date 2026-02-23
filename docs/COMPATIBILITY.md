# Compatibility Specification

---

## 1. Purpose

This document defines compatibility guarantees and constraints for:

* Programming languages
* Repository structures
* Build systems
* Operating systems
* Storage backends
* IDE integrations
* LSIF / SCIP import formats
* MCP clients
* Schema versions

Compatibility must be:

* Explicit
* Versioned
* Backward-aware
* Forward-safe where possible
* Deterministic

Compatibility must not:

* Be assumed implicitly
* Break silently between versions
* Depend on undocumented behavior

---

## 2. Language Compatibility

### 2.1 Supported Languages

Initial support:

* TypeScript / JavaScript
* Python
* Go
* C#
* Rust

Additional languages must implement:

* Tree-sitter adapter
* Symbol extractor
* Reference extractor
* Visibility rules
* UID normalization logic

Core system must remain language-agnostic.

---

### 2.2 Multi-Language Projects

System must support:

* Mixed-language repositories
* Multiple Tree-sitter grammars
* Multiple LSP servers
* Multiple LSIF/SCIP imports

UID format must include language prefix.

No cross-language UID collisions allowed.

---

## 3. Repository Compatibility

System must support:

* Monorepos
* Multi-module repositories
* Nested package structures
* Custom source directories
* Exclusion patterns
* Generated file exclusion

Repository root must be configurable.

Sidecar directory location must be configurable.

---

## 4. Build System Compatibility

System must not depend on:

* Specific build system
* Specific compiler
* Project compilation
* Build artifacts

LSP or LSIF may depend on build system externally, but core index must not.

Supported build systems:

* npm / pnpm / yarn
* pip / poetry
* cargo
* go modules
* dotnet
* gradle / maven (if LSIF used)

Core must function even if project does not build.

---

## 5. Operating System Compatibility

System must support:

* Linux
* macOS
* Windows

File path normalization rules:

* Normalize path separators
* Normalize case sensitivity (where necessary)
* Use project-relative paths
* Avoid absolute path storage

UID must not depend on OS-specific path representation.

---

## 6. Storage Backend Compatibility

Supported backends:

* SQLite (default)
* RocksDB
* LMDB
* Key-value store

Logical schema must remain independent.

Switching storage backend must not change:

* UID
* Ranking
* Determinism
* Anchor behavior

Storage engine swap must not require doc rewrite.

---

## 7. Schema Version Compatibility

Each index must store:

* schema_version
* uid_format_version
* ranking_version
* anchor_algorithm_version

When version mismatch detected:

* Attempt migration if safe
* Else require full reindex
* Never silently ignore mismatch

Backward compatibility must be maintained when possible.

---

## 8. UID Format Compatibility

UID format must:

* Be versioned
* Remain stable once released
* Only change with major version bump

If UID format changes:

* Provide migration tool
* Provide mapping file
* Require anchor revalidation

Never silently reinterpret UID.

---

## 9. Sidecar Format Compatibility

Sidecar format must:

* Remain Markdown + YAML
* Remain backward-compatible where possible
* Support additional metadata fields safely
* Ignore unknown metadata fields gracefully

Breaking metadata changes require version bump.

Old documents must not be invalidated silently.

---

## 10. LSIF / SCIP Compatibility

System must:

* Support known LSIF schema versions
* Support known SCIP schema versions
* Reject incompatible schema versions
* Log explicit compatibility error

External index formats must not override core UID determinism.

---

## 11. MCP Protocol Compatibility

MCP tools must:

* Remain backward-compatible within major version
* Avoid breaking parameter schema
* Maintain deterministic response structure
* Preserve field names across minor versions

If tool signature changes:

* Introduce new tool name
* Deprecate old tool gradually

Never silently remove fields.

---

## 12. CLI Compatibility

CLI must:

* Preserve command names across minor versions
* Preserve exit codes
* Preserve JSON output structure
* Avoid breaking scripts

Breaking changes require major version bump.

---

## 13. IDE Plugin Compatibility

VS Code and JetBrains plugins must:

* Remain compatible with stable MCP version
* Detect MCP version mismatch
* Display compatibility warning
* Disable incompatible features safely

Plugin must not crash due to version mismatch.

---

## 14. Ranking Compatibility

Ranking algorithm version must be tracked.

If ranking weights change:

* Version must increment
* Determinism must remain
* Pagination stability must remain

Ranking change must not break API contract.

---

## 15. Token Economy Compatibility

Token economy safeguards must:

* Remain stable across versions
* Not unexpectedly increase default limits
* Not silently expand query size

If defaults change:

* Must be documented
* Must not break existing automation

---

## 16. Backward Compatibility Policy

Policy:

* Minor version → backward-compatible
* Patch version → bug fixes only
* Major version → may break compatibility

Breaking changes must:

* Be documented
* Provide migration guide
* Provide tooling support where possible

---

## 17. Forward Compatibility Policy

System must:

* Ignore unknown metadata fields
* Ignore unknown optional JSON fields
* Fail gracefully on unknown required fields
* Allow gradual feature rollout

Future versions must not break older sidecar files abruptly.

---

## 18. CI Compatibility

System must support:

* Headless indexing
* Deterministic rebuild in CI
* No interactive prompts
* Exit codes for failure
* Read-only MCP server mode

CI must produce reproducible index.

---

## 19. Plugin Ecosystem Compatibility

Future plugins must:

* Use MCP exclusively
* Not access index storage directly
* Respect UID format
* Respect ranking determinism
* Respect token limits

Core must not allow bypass of compatibility guarantees.

---

## 20. Non-Goals

Compatibility specification does not:

* Guarantee compatibility across unrelated forks
* Guarantee compatibility across unsupported language grammars
* Guarantee compatibility with unstable LSP servers
* Guarantee compatibility with corrupted LSIF/SCIP data

Unsupported environments must fail clearly.

---

## 21. Compatibility Matrix (Initial)

| Component  | Linux | macOS | Windows |
| ---------- | ----- | ----- | ------- |
| Core Index | Yes   | Yes   | Yes     |
| MCP Server | Yes   | Yes   | Yes     |
| CLI        | Yes   | Yes   | Yes     |
| VS Code    | Yes   | Yes   | Yes     |
| JetBrains  | Yes   | Yes   | Yes     |

Language support depends on Tree-sitter and LSP availability.

---

## 22. Testing Compatibility

Must test:

* Cross-OS UID stability
* Cross-backend determinism
* Sidecar backward compatibility
* CLI script stability
* MCP schema stability
* Plugin version mismatch handling

Compatibility regressions must fail CI.

---

## 23. Summary

Compatibility guarantees:

* Stable UID identity
* Stable schema
* Stable CLI interface
* Stable MCP tools
* Stable ranking
* Stable sidecar format
* Multi-language support
* Multi-OS support

Breaking compatibility must be deliberate, documented, and versioned.

Compatibility is a contract with users.
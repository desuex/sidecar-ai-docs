# Implementation Stack Decision

This document selects the implementation stack for the Sidecar system (AI-native, refactor-resilient, sidecar documentation + index) with **portability** as a first-class requirement.

Primary goals driving these choices:

* Determinism (stable ordering, stable IDs, stable schemas)
* Performance on large repositories
* Safe-by-default (no project code execution)
* Cross-platform support (Linux/macOS/Windows)
* Thin IDE clients over MCP
* Simple distribution (ideally a single core binary)

---

## 1. Stack Decision (Chosen)

**Core (indexer + storage + MCP server + CLI):** **Rust**

**Parsing:** **Tree-sitter** (Rust bindings)

**Storage (default):** **SQLite**

**MCP transport:** **STDIO JSON-RPC** (primary), **HTTP** (optional later)

**VS Code extension:** **TypeScript**

**JetBrains plugin:** **Kotlin**

**Optional enrichment (opt-in):**

* LSP adapters (external language servers)
* SCIP/LSIF import (external index formats)

This is the “single local daemon/binary + deterministic DB + thin clients” architecture.

---

## 2. Portability Commitments

### 2.1 Supported Platforms

The core toolchain must run on:

* Linux (x86_64, aarch64)
* macOS (x86_64, arm64)
* Windows (x86_64)

All paths stored in the index must be **repo-relative** and normalized.

### 2.2 Distribution Targets

**Primary distribution:** a single executable named `sidecar`.

Packaging goals:

* No dependency on a system database server
* No dependency on Node/Python/.NET runtimes for the core
* Minimal dynamic libraries (prefer static linking where feasible)

Recommended distribution channels:

* GitHub Releases (prebuilt binaries)
* Homebrew (macOS/Linux)
* Scoop / winget (Windows)
* Cargo install (developer convenience, not primary)

### 2.3 Runtime and Tooling Constraints

Core must not:

* Execute project code
* Invoke build systems implicitly
* Require project to compile
* Require internet access

Optional semantic enrichment (LSP/SCIP/LSIF) is opt-in and must degrade gracefully.

### 2.4 Deterministic Cross-OS Behavior

Portability requires deterministic behavior across OSes.

Therefore:

* Use explicit stable sorting everywhere
* Avoid non-deterministic map iteration
* Normalize path separators and casing rules
* Use versioned canonicalization rules
* Use fixed hash algorithms and stable encodings

---

## 3. Core Implementation Stack

### 3.1 Language: Rust

Why Rust:

* High performance for indexing, graph queries, diffing
* Strong safety guarantees for untrusted input
* Good cross-platform single-binary distribution
* Strong Tree-sitter integration via FFI
* Easier enforcement of determinism constraints

Rust is used for:

* Indexer (parse → symbols → refs)
* UID + fingerprint computation
* Anchor/selector matching + confidence scoring
* AST diff + rebase engine
* Storage layer + migrations
* Ranking engine
* MCP server
* CLI

---

### 3.2 Storage: SQLite (default)

Why SQLite:

* Portable, robust, widely available
* No server process required
* Strong transactional guarantees
* Works in local dev, CI, and IDE workflows

Usage:

* Tables for symbols, references, docs, anchors, migrations
* Indexed columns for fast lookup
* Deterministic ordering enforced at query layer
* Optional WAL mode

Future (optional):

* LMDB / RocksDB for extreme scale
* Dual-store (SQLite metadata + KV blobs) if required

---

### 3.3 Parsing: Tree-sitter

Why Tree-sitter:

* Fast incremental parsing
  n- Large grammar ecosystem across languages
* Structural AST access for fingerprinting/selectors
* No code execution

Implementation:

* Rust crate `tree-sitter` + grammar crates
* Per-language adapter layer:

  * Symbol extraction
  * Visibility rules
  * Reference extraction heuristics (AST-only baseline)
  * Normalization rules for fingerprinting

AST-only mode must be fully functional without LSP.

---

## 4. Optional Enrichment Layers

### 4.1 LSP Adapters (opt-in)

Purpose:

* Improve reference accuracy and symbol resolution

Constraints:

* Must not compromise determinism
* Must validate all returned locations and ranges
* Must preserve “AST-only mode” as baseline

Integration:

* Run external language server(s) as subprocesses
* Query: definition, references, document symbols
* Normalize into internal UID/xref model

---

### 4.2 SCIP / LSIF Import (opt-in)

Purpose:

* Reuse existing CI-produced indexes (e.g., Sourcegraph SCIP)

Constraints:

* Import must not override internal UID determinism
* Must store provenance metadata
* Must validate external input strictly

---

## 5. MCP + CLI

### 5.1 MCP Server

Implementation: Rust.

Transport:

* Primary: STDIO JSON-RPC
* Optional later: HTTP

Requirements:

* Strict schemas
* Field selection
* Pagination
* Token-aware truncation flags
* Deterministic output

Server modes:

* `sidecar mcp` (stdio)
* `sidecar mcp --http :port` (optional later)

---

### 5.2 CLI

Implementation: Rust.

Requirements:

* `--json` for structured output
* Stable ordering
* Strict exit codes
* Scriptable and pipe-friendly

Distribution: bundled with the core binary.

---

## 6. IDE Integration

### 6.1 VS Code Extension

Language: TypeScript.

Communication:

* Spawn `sidecar mcp` via STDIO
* JSON-RPC client
* Aggressive caching for hover/decorations

---

### 6.2 JetBrains Plugin

Language: Kotlin.

Communication:

* Spawn `sidecar mcp` via ProcessBuilder (STDIO)
* Async requests (coroutines)
* Never block EDT

---

## 7. Hashing, Fingerprinting, Determinism

### 7.1 Hash Functions

Requirements:

* Consistent across OS/CPU
* Stable encodings
* Versioned fingerprint scheme

Recommendation:

* BLAKE3 for fingerprints (truncate + base32/base64url)
* Maintain `fingerprint_version` in index metadata

### 7.2 Canonicalization Rules

Determinism depends on:

* Canonical path normalization
* Canonical AST normalization
* Stable serialization order
* Stable ranking tie-breakers

All canonicalization rules must be versioned.

---

## 8. Alternatives Considered (Summary)

* TypeScript/Node core: faster iteration, worse performance/distribution
* Go core: viable, Rust preferred for Tree-sitter + safety constraints
* JVM core: good for plugins, not ideal for core portability

---

## 9. Final Notes

This stack is chosen to:

* Keep the core portable and easy to install
* Preserve strict determinism guarantees
* Avoid executing untrusted project code
* Enable thin IDE clients that do not reimplement logic

Optional enrichment layers (LSP/SCIP/LSIF) must remain optional and must not change the correctness or determinism of the core.

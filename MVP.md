# MVP.md

## 0. Goal

Deliver a **working, portable, deterministic** “Sidecar core” that proves:

* **Token economy** via bounded structured retrieval (MCP + CLI)
* **Drop-in** indexing of a repo without modifying source code
* **Useful** symbol + reference navigation for AI agents and humans

Non-goal: perfect semantic references for every language on day 1.

---

## 1. MVP Scope

### 1.1 Must Have (MVP = shippable)

**Core**

* Single binary: `sidecar`
* Cross-platform builds: Linux/macOS/Windows
* Deterministic behavior across runs (stable ordering, stable UIDs)

**Indexing**

* Tree-sitter parsing
* Symbol extraction (minimal but correct)
* UID generation v1
* Basic reference extraction (AST-only) for at least one language
* Incremental indexing for changed files (simple “file hash changed → reindex file” is OK)

**Storage**

* SQLite database in `.sidecar/index.sqlite` (or configurable)
* Schema version stored in DB
* Atomic writes / transactions

**MCP (stdio)**

* Deterministic JSON responses
* Pagination + field selection
* Strict validation of inputs (limits, uid regex, etc.)
* No implicit expansion

**CLI**

* `sidecar index` (build/update index)
* `sidecar search` (symbols)
* `sidecar symbol` (symbol details)
* `sidecar refs` (find references)
* `--json` outputs for all commands
* Stable exit codes

**Docs Sidecar**

* Detect sidecar root directory (default `.sidecar/` or `docs-sidecar/`)
* Minimal doc lookup by UID (even if docs are sparse initially)
* `sidecar doc <uid>` reads sidecar markdown by UID mapping

---

### 1.2 Should Have (still MVP if delayed)

* “Project status” command: `sidecar status`
* “Index stale” detection via git/mtime/hash
* Basic ranking for `search` and `refs` (lexical + simple heuristics)
* Fast path caches in MCP server for hover-like workflows

---

### 1.3 Nice to Have (post-MVP)

* LSP adapters (semantic references)
* SCIP/LSIF import
* Anchor rebinding / AST diff
* VS Code extension
* JetBrains plugin
* Coverage metrics
* Selector model for non-symbol anchors

---

## 2. MVP Language Support

### 2.1 Pick One Primary Language (v0)

Choose **one** to make it real end-to-end.

Recommended MVP-first language (pragmatic):

* **TypeScript/JavaScript** (huge demand, agents struggle with JS repos)

Alternative if you want easiest semantics early:

* **Go** (simple project structure, easier symbol/ref resolution in many cases)

MVP requirement:

* One language supported end-to-end with “good enough” results:

  * symbols
  * UIDs
  * references

Others can be parse-only (symbols without refs) initially.

---

## 3. MVP User Stories

### US-1: Find what a symbol is

As a developer/agent, I can search symbols and get a stable UID + short metadata.

**Flow**

* `sidecar search "CartService.calculateTotal" --json`
* choose uid
* `sidecar symbol <uid> --json`

### US-2: Find where a symbol is used

As a developer/agent, I can request top-N references, deterministically sorted.

**Flow**

* `sidecar refs <uid> --limit 20 --json`

### US-3: Get minimal documentation without dumping files

As an agent, I can fetch doc summary by UID (or “missing”) without reading the whole file.

**Flow**

* MCP: `get_documentation(uid, mode="summary")`

### US-4: Integrate in agent workflow

As an agent tool, I can use MCP stdio to retrieve bounded data and avoid token blowups.

---

## 4. MVP Commands

### 4.1 CLI Surface

#### `sidecar index`

Builds or updates the index for a repo.

Examples:

* `sidecar index`
* `sidecar index --root .`
* `sidecar index --sidecar-dir .sidecar`
* `sidecar index --languages ts,js`

Outputs:

* human: progress + summary
* `--json`: `{ indexed_files, symbols, refs, duration_ms, schema_version }`

Exit codes:

* 0 success
* 2 validation error
* 3 index corrupted/incompatible schema
* 4 parse error (non-fatal if configured)
* 5 internal error

---

#### `sidecar search <query>`

Search symbols deterministically.

Flags:

* `--limit N` (default 20)
* `--offset N` (default 0)
* `--fields uid,qualified_name,kind,score` (field selection)
* `--json`

---

#### `sidecar symbol <uid>`

Return symbol metadata.

Flags:

* `--fields ...`
* `--json`

---

#### `sidecar refs <uid>`

Return references to symbol.

Flags:

* `--limit N` (default 20)
* `--offset N` (default 0)
* `--fields ...`
* `--json`

---

#### `sidecar doc <uid>`

Fetch sidecar documentation entry (if exists).

Flags:

* `--mode summary|full` (default summary)
* `--json`

---

#### `sidecar mcp`

Starts MCP server on stdio.

Flags:

* `--root .`
* `--sidecar-dir .sidecar`
* `--log-level info|debug`
* `--json-logs`

---

## 5. MVP MCP Tools

Minimum MCP toolset:

1. `search_symbols`

* input: `{ query, limit, offset, fields }`
* output: `{ results: [...], truncated }`

2. `get_symbol`

* input: `{ uid, fields }`
* output: `{ symbol }`

3. `find_references`

* input: `{ uid, limit, offset, fields }`
* output: `{ total, results: [...], truncated }`

4. `get_documentation`

* input: `{ uid, mode: "summary"|"full", max_chars?, fields? }`
* output: `{ exists, summary?, content?, truncated, anchor_confidence? }`

All must enforce:

* limits
* schema validation
* stable ordering
* deterministic tie-breakers

---

## 6. Data Model (MVP Minimal)

### 6.1 Tables (SQLite)

* `meta` (schema_version, uid_format_version, fingerprint_version, created_at)
* `files` (file_uid, path, language, content_hash, last_indexed_at)
* `symbols` (uid, file_uid, kind, qualified_name, name, visibility, fingerprint, range_start, range_end)
* `refs` (from_uid, to_uid, file_uid, range_start, range_end, ref_kind)
* `docs` (doc_uid, target_uid, path, summary_cache, updated_at)

MVP ranges can be byte offsets; add line/col later.

---

## 7. Determinism Requirements (MVP Gate)

MVP is **not accepted** unless:

* UIDs stable under formatting-only edits
* Search ordering stable across runs
* Refs ordering stable across runs
* Pagination stable (page1+page2 == first N)
* JSON output stable field order (or canonical serialization)
* No nondeterministic iteration over hash maps in output

---

## 8. Token Economy Requirements (MVP Gate)

MVP is **not accepted** unless:

* Default `limit` is bounded (≤ 20)
* `get_documentation` returns summary by default
* No implicit expansion (no file dumps)
* Field selection supported for list queries
* Responses include `truncated: true` when applicable

---

## 9. Security Requirements (MVP Gate)

MVP is **not accepted** unless:

* Strict UID validation
* Path traversal protection for sidecar file access
* Safe YAML parsing mode (if YAML used at all in MVP)
* Input size limits (files, docs, requests)
* No project code execution, no builds invoked

---

## 10. MVP Milestones

### M0: Skeleton ✅ Complete

* Repo layout
* Rust workspace (6 crates: types, parsing, core, storage, mcp, cli)
* CLI wiring (clap with 6 subcommands)
* SQLite schema + migrations (5 tables, version tracking)
* Basic logging (tracing to stderr)
* CI pipeline (.github/workflows/ci.yml — 3-OS matrix)
* 29 unit tests, cargo fmt/clippy clean

### M1: Parse + Symbols ✅ Complete

* Tree-sitter integration for TypeScript (v0.23)
* Symbol extraction (classes, methods, functions, interfaces, enums, type aliases, variables)
* UID + fingerprint v1 (BLAKE3, deterministic)
* Incremental indexing (content hash skip)
* `sidecar index`, `sidecar search`, `sidecar symbol` — all with `--json`
* 43 tests (unit + integration), insta snapshots
* Determinism verified: same UIDs across runs, stable search ordering

### M2: References

* AST-only ref extraction (best effort)
* `sidecar refs`
* Ranking v0 + deterministic tie-breakers

### M3: MCP

* `sidecar mcp` stdio server
* MCP tool set (search/get_symbol/find_references/get_documentation)

### M4: Sidecar Docs Minimal

* sidecar dir detection
* doc lookup by UID mapping
* `sidecar doc`, MCP `get_documentation`

### M5: MVP Hardening

* determinism tests
* adversarial input tests
* performance smoke test on medium repo
* release artifacts

---

## 11. MVP Success Criteria

MVP is successful if:

* Can index a repo and answer symbol/ref queries via CLI and MCP
* Default queries are bounded and token-efficient
* Deterministic outputs pass snapshot tests
* Cross-platform packaging works
* Docs sidecar can attach at least by UID mapping (even if no rebinding yet)
* Agent can complete “find symbol → get summary → find usages” without opening full files

---

## 12. Out of Scope (Explicit)

Not in MVP:

* Anchor rebinding, AST diff, migration events
* Selector model for non-symbol nodes
* “Perfect references” for dynamic languages
* IDE plugins
* Semantic embeddings
* Remote/cloud service
* Multi-repo linking
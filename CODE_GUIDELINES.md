# CODE_GUIDELINES.md

## 0. Purpose

These guidelines define how code is written and reviewed in this repository.

Primary objectives:

* **Determinism** (same input → same output)
* **Correctness** (no silent failures)
* **Portability** (Linux/macOS/Windows)
* **Maintainability** (small, composable modules)
* **Token economy discipline** (bounded outputs, progressive disclosure)

This is an infrastructure project. “It works on my machine” is not acceptable.

---

## 1. Core Engineering Principles

### 1.1 TDD (Test-First) as Default

We use **test-first** for all logic that impacts:

* UID / fingerprint generation
* ranking and ordering
* anchoring / selector matching
* migrations and schema changes
* MCP response shapes
* CLI JSON outputs
* security boundaries (validation, traversal, limits)

Workflow:

1. Write a failing test (unit or snapshot).
2. Implement the smallest change to pass it.
3. Refactor only after green.

If you can’t write a test, the feature is probably underspecified.

---

### 1.2 KISS (Keep It Simple)

Prefer:

* simple data structures
* explicit control flow
* obvious ordering and sorting
* clear invariants
* minimal dependencies

Avoid:

* clever abstractions
* magic heuristics without versioning
* “auto” behavior that hides costs

Simplicity is a feature.

---

### 1.3 Minimal Change Priority

When modifying behavior:

* keep the change **small**
* keep the diff **localized**
* avoid touching unrelated files
* avoid opportunistic refactors in the same PR

Large refactors must be isolated into dedicated PRs.

---

### 1.4 SOLID, but Not Dogma

Use SOLID to prevent coupling, but avoid “architecture astronautics”.

* **S**: keep modules single-purpose
* **O**: extend via adapters (language adapters, storage adapters)
* **L**: keep interfaces honest and substitutable
* **I**: small trait interfaces (don’t force implementers to do everything)
* **D**: depend on abstractions at boundaries, concrete types inside

If SOLID makes code harder to read, you overdid it.

---

### 1.5 “Make Invalid States Unrepresentable”

Prefer types and enums that encode invariants:

* validated UID types (not `String`)
* bounded `Limit` types (not raw `u32`)
* strict `PathRel` wrapper for repo-relative paths
* structured request/response types with schema validation

If a value must be validated, model it as a validated type.

---

## 2. Determinism Rules (Non-Negotiable)

### 2.1 Stable Ordering Everywhere

Never rely on:

* `HashMap` iteration order
* filesystem directory iteration order
* “whatever the DB returns”
* non-stable sorts

Always:

* specify `ORDER BY` in SQL
* use `BTreeMap` when ordering matters
* explicitly `sort_by` with deterministic tie-breakers

---

### 2.2 Canonicalization Is Versioned

Any rule that affects identity must be versioned:

* UID format
* fingerprint algorithm
* AST normalization rules
* path normalization

Never silently change these.

---

### 2.3 No Hidden Randomness

Do not use:

* random numbers
* current time
* environment-dependent defaults
* locale-dependent formatting

If timestamps are needed, keep them out of deterministic outputs (or behind flags).

---

## 3. Token Economy Rules (Non-Negotiable)

### 3.1 Progressive Disclosure

Default responses must be minimal:

* summary instead of full docs
* top-N instead of full reference list
* metadata before content

Expansion must be explicit via parameters.

---

### 3.2 Bounded Queries

Every query must have:

* `limit` (default, max)
* `offset` (or cursor)
* optional `depth` (if graph traversal exists)

Never implement unbounded traversal.

---

### 3.3 Output Budget Awareness

Prefer responses that are:

* structured JSON
* compact (no verbose prose in APIs)
* truncation-aware (`truncated: true`)

---

## 4. Security Rules (Non-Negotiable)

### 4.1 All Inputs Are Untrusted

Treat as untrusted:

* repo source files
* sidecar docs
* LSIF/SCIP imports
* MCP inputs
* CLI args

Validate strictly at boundaries.

---

### 4.2 No Project Code Execution

Do not:

* import user code
* run build steps
* execute scripts
* run package managers

Only parse text.

---

### 4.3 Path Safety

All filesystem paths must be:

* normalized
* repo-relative
* validated against traversal
* never allow `../` escapes

Errors must not leak sensitive absolute paths unnecessarily.

---

## 5. Architecture Boundaries

### 5.1 Layering

Recommended layering:

* **parsing**: AST, symbols, ranges
* **core**: UID, fingerprints, ranking, anchoring logic
* **storage**: schema + persistence
* **mcp**: protocol boundary + tool handlers
* **cli**: UX boundary

Rules:

* core must not depend on CLI or MCP
* storage must not leak SQL into core types (use repositories/queries)
* parsing adapters must be isolated by language module

---

### 5.2 Adapters Over Conditionals

For language-specific behavior:

* add a language adapter module
* do not sprinkle `if language == ...` across core

For storage backends:

* use a storage trait boundary
* SQLite is default, but core should be backend-agnostic

---

## 6. Testing Standards

### 6.1 What Must Be Tested

Must have tests for:

* UID/fingerprint determinism
* formatting-only stability
* ranking stability and tie-breakers
* pagination stability (page composition equivalence)
* CLI `--json` outputs (snapshots)
* MCP tool outputs (snapshots)
* input validation (negative tests)
* path traversal rejection (negative tests)

---

### 6.2 Snapshot Testing Discipline

Snapshots must be:

* deterministic
* stable sorted
* sanitized (no timestamps, no absolute paths)

If a snapshot changes:

* explain why in PR description
* ensure change is intended

---

### 6.3 Performance Tests

At minimum:

* smoke perf test on a medium fixture repo
* ensure no accidental O(n²) loops in hot paths

Performance regressions must be justified.

---

## 7. Error Handling & Logging

### 7.1 Errors Are Data

Prefer structured errors:

* error codes for CLI/MCP
* `thiserror` enums internally
* attach context at boundaries

No panics for expected failures.

---

### 7.2 Logging

Use structured logging (`tracing`) with levels:

* `error`: failed request or corrupted state
* `warn`: degraded but continuing
* `info`: major lifecycle events
* `debug/trace`: detailed internals behind flags

Do not log:

* full source file contents
* full documentation bodies
* secrets

---

## 8. Review Checklist (PR Gate)

A PR should be accepted only if:

* tests exist and are meaningful
* outputs remain deterministic
* limits are enforced everywhere
* no new hidden heuristics
* no silent behavior changes
* security boundaries are respected
* documentation/spec updated if behavior changes
* minimal diff principle observed

---

## 9. “Versioned Heuristics” Policy

Some heuristics are inevitable (ranking, rebinding later).

Rule:

* every heuristic must be versioned
* heuristics must be deterministic
* heuristics must expose confidence/score, not hide it
* heuristics must never silently overwrite user intent

---

## 10. Dependency Policy

Prefer:

* fewer dependencies
* well-maintained crates
* crates with stable APIs
* crates without hidden network behavior

Avoid:

* huge framework deps
* macros that obscure control flow
* runtime plugin execution in core

---

## 11. Documentation of Code

We document:

* invariants
* tricky ordering decisions
* canonicalization rules
* security decisions
* performance tradeoffs

We do not write long essays in comments.

If it’s a design decision, put it in `docs/`.

---

## 12. Philosophy

This project succeeds if it becomes boring:

* boringly deterministic
* boringly safe
* boringly portable
* boringly predictable

Agents and humans should trust it.

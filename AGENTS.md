# AGENTS.md

## Purpose

This file describes how AI agents (and humans) should scaffold the Sidecar project repository so it is:

* portable (Linux/macOS/Windows)
* deterministic
* testable from day 1
* CI-ready
* friendly to contributors

This is a **scaffold guide**: it creates the project shape, build config, quality tools, and baseline tests.

---

## 0. One-Sentence Rule

Prefer a **single Rust workspace** with **one core binary** (`sidecar`) plus internal crates, strict formatting/linting, deterministic snapshot tests, and CI that builds on all OSes.

---

## 1. Repository Layout (Scaffold)

Create this structure:

```text
sidecar/
  README.md
  LICENSE
  CHANGELOG.md
  SECURITY.md
  CODE_OF_CONDUCT.md
  CONTRIBUTING.md
  GOVERNANCE.md

  AGENTS.md
  MVP.md
  STACK.md

  docs/                   # specs and design docs (already seeded)
    ...

  crates/
    sidecar/              # main binary crate
      Cargo.toml
      src/
        main.rs
        lib.rs            # optional: expose internal API
        cli.rs
        exit_codes.rs
        version.rs

    core/                 # core domain logic (indexing, uid, ranking)
      Cargo.toml
      src/
        lib.rs
        errors.rs

    storage/              # sqlite schema, queries, migrations
      Cargo.toml
      src/
        lib.rs
        schema.rs
        migrations.rs

    parsing/              # tree-sitter integration + language adapters
      Cargo.toml
      src/
        lib.rs
        language/
          mod.rs
          ts.rs           # chosen MVP language adapter (example)

    mcp/                  # MCP stdio server
      Cargo.toml
      src/
        lib.rs
        server.rs
        protocol.rs
        tools/
          mod.rs
          search_symbols.rs
          get_symbol.rs
          find_references.rs
          get_documentation.rs

  tests/
    vectors/              # fixture repos + before/after refactor cases
      README.md
    snapshots/            # snapshot outputs for determinism tests
      .gitkeep

  scripts/
    dev/
      install-hooks.sh
    ci/
      smoke.sh

  .github/
    workflows/
      ci.yml

  .editorconfig
  .gitignore
  rust-toolchain.toml
  Cargo.toml              # workspace root
  Cargo.lock              # committed
```

Notes:

* Keep IDE plugins in a separate repo later (or under `integrations/`), not in MVP.
* Keep core logic in `crates/core` so the CLI and MCP can share it.

---

## 2. Rust Workspace Setup

### 2.1 Root `Cargo.toml` (workspace)

* Use edition 2021 (or latest stable if you prefer, but lock it).
* Put shared deps in `[workspace.dependencies]`.
* Enable `resolver = "2"`.

### 2.2 Toolchain pinning

Create `rust-toolchain.toml` with:

* stable channel
* components: rustfmt, clippy

Pinning avoids “it works on my machine” drift.

---

## 3. Core Dependencies (Recommended)

Use a minimal, stable set:

* CLI: `clap` (derive)
* Logging: `tracing`, `tracing-subscriber`
* Errors: `thiserror`, `anyhow` (use sparingly at boundaries)
* JSON: `serde`, `serde_json`
* SQLite: `rusqlite` **or** `sqlx` (choose one; MVP prefers `rusqlite` for simplicity)
* Hash: `blake3`
* Tree-sitter: `tree-sitter` + language grammar crates (e.g., `tree-sitter-typescript`)
* Testing:

  * `insta` for snapshot tests
  * `pretty_assertions` (optional)
  * `tempfile`

Security for YAML (if used in MVP):

* `serde_yaml` in strict mode (avoid advanced YAML features)
* or avoid YAML in MVP and use TOML/JSON front matter first

---

## 4. Quality Gates (Non-Negotiable)

### 4.1 Formatting

* `rustfmt` required
* CI fails on formatting mismatch

### 4.2 Linting

* `clippy` required
* CI fails on clippy warnings (prefer `-D warnings`)

### 4.3 Determinism Rules

Agents must enforce:

* stable sorting before output
* no iteration over `HashMap` for output ordering
* canonical serialization for JSON outputs where needed

Prefer:

* `BTreeMap` for maps that must have stable order
* explicit `sort_by` for vectors

---

## 5. Baseline Test Suite (Day 1)

Create tests before features grow.

### 5.1 Unit tests

Minimum:

* UID generation determinism
* fingerprint stability under formatting-only change (fixture-based)
* ranking tie-break determinism

### 5.2 Snapshot tests

Use `insta` snapshots for:

* `sidecar search` JSON output
* `sidecar refs` JSON output
* MCP tool outputs

Snapshots must be:

* stable
* sorted
* stripped of timestamps and machine-specific paths

### 5.3 Golden fixture repos

Add small fixture repos under `tests/vectors/`:

* minimal TS repo (or chosen MVP language)
* include a few files with symbol references

---

## 6. CI (GitHub Actions)

Create `.github/workflows/ci.yml`:

Matrix:

* ubuntu-latest
* macos-latest
* windows-latest

Steps:

1. checkout
2. install rust toolchain (via `dtolnay/rust-toolchain` or `actions-rs`)
3. `cargo fmt --check`
4. `cargo clippy --all-targets --all-features -- -D warnings`
5. `cargo test --all --all-features`
6. smoke script: index fixture repo + run a couple CLI commands

Optional:

* cache cargo registry and target directory

CI must not require network other than fetching crates.

---

## 7. Git Hooks (Optional but Helpful)

Provide `scripts/dev/install-hooks.sh` to install:

* pre-commit: `cargo fmt`
* pre-push: `cargo test` (optional)

Do not make hooks mandatory, but document them.

---

## 8. Developer UX Defaults

### 8.1 CLI UX

* All commands support `--json`
* Human output should be concise and stable
* Exit codes defined in `crates/sidecar/src/exit_codes.rs`
* No panic in normal control flow

### 8.2 Logging

* default log level: info
* `RUST_LOG` support via `tracing-subscriber`
* JSON logs optional

### 8.3 Config

For MVP, prefer:

* flags + env vars
* minimal config file support later

---

## 9. Build & Release (Portability)

For release artifacts:

* use `cargo build --release`
* optionally integrate `cargo-dist` later (post-MVP) to generate installers

For Windows:

* ensure paths are normalized and repo-relative
* avoid unix-only assumptions in scripts

---

## 10. What Agents Must Not Do

* Do not add runtime dependency on Node/Python/.NET for the core
* Do not execute project code to extract docs (no autodoc import)
* Do not add unbounded queries (always enforce limits)
* Do not add nondeterministic ordering
* Do not add implicit “dump full file” outputs

---

## 11. Scaffold Checklist

An agent completing the scaffold must verify:

* `cargo fmt --check` passes
* `cargo clippy -D warnings` passes
* `cargo test` passes
* CI pipeline runs on all OS
* `sidecar --help` works
* minimal snapshot test exists
* fixture repo exists
* repo builds offline after deps are fetched

---

## 12. First Implementation Targets (After Scaffold)

After scaffold, implement in this order:

1. UID + fingerprint module (unit tests)
2. Tree-sitter parse + symbol extraction (fixture tests)
3. SQLite schema + insert/query
4. CLI: `index`, `search`, `symbol`
5. refs extraction + `refs` command
6. MCP stdio server + 4 tools

# Sidecar Usage Scenarios

This document tracks practical usage scenarios in two states:

- Implemented: available in the current CLI.
- Planned: specified in docs, not yet implemented.

## Implemented Scenarios

### 1) Build an index for a repository

Use this when you want searchable symbol/reference data.

```bash
sidecar --root . index --json
```

### 2) Search for symbols by name

Use this when you know a symbol name fragment.

```bash
sidecar --root . search CartService --limit 20 --offset 0 --json
```

### 3) Inspect a symbol by UID

Use this when you already have a UID and need details.

```bash
sidecar --root . symbol sym:ts:src/cart:CartService:866eb7ea --json
```

### 4) Find references to a symbol

Use this for impact analysis and dependency tracing.

```bash
sidecar --root . refs sym:ts:src/cart:CartService:866eb7ea --limit 50 --offset 0 --json
```

### 5) Read sidecar documentation for a symbol

Use summary mode for quick context and full mode for full text.

```bash
sidecar --root . doc sym:ts:src/cart:CartService:866eb7ea --mode summary --json
sidecar --root . doc sym:ts:src/cart:CartService:866eb7ea --mode full --json
```

### 6) Run MCP server over stdio

Use this when integrating Sidecar with MCP-capable clients/tools.

```bash
sidecar --root . mcp --log-level info --json-logs
```

### 7) Export sidecar docs to MkDocs/RTD

Use this for docs publishing pipelines. This runs structured rendering plus anchor validation report generation.

```bash
sidecar export mkdocs --root . --out docs/generated
```

Optional custom index DB path:

```bash
sidecar export mkdocs --root . --out docs/generated --index-db .sidecar/index.sqlite
```

### 8) Drive self-documentation targets via MCP

Use this to measure current documentation coverage and pull a bounded queue of undocumented symbols.

```bash
printf '%s\n' \
'{"jsonrpc":"2.0","id":1,"method":"coverage_metrics","params":{"public_only":true,"scan_limit":5000}}' \
| sidecar --root . mcp --log-level error
```

```bash
printf '%s\n' \
'{"jsonrpc":"2.0","id":2,"method":"detect_undocumented_symbols","params":{"public_only":true,"scan_limit":5000,"limit":25,"offset":0}}' \
| sidecar --root . mcp --log-level error
```

Use the returned symbol UIDs with `get_symbol`, `find_references`, and `get_documentation` to draft focused updates.

### 9) Run quality gates locally

Use this before pushing changes.

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --all-features
./scripts/ci/coverage.sh
./scripts/ci/doc_coverage_gate.sh
./scripts/ci/mcp_smoke.sh
mkdocs build --strict
```

## Planned Usage Scenarios

These scenarios are planned in specs, but not implemented yet.

### 1) Index lifecycle commands

- `sidecar init`
- `sidecar update`
- `sidecar status`
- `sidecar reset`

### 2) Extended query workflows

- `sidecar search-symbols "query"` (kind-aware search)
- `sidecar impact <uid>` (transitive impact view)

### 3) Documentation authoring and binding workflows

- `sidecar doc-create --symbol <uid> --title "..."`
- `sidecar doc-attach <doc_uid> --symbol <uid>`
- `sidecar doc-detach <doc_uid> --symbol <uid>`
- `sidecar doc-list <uid>`
- `sidecar doc-search "query"`

### 4) Anchor maintenance workflows

- `sidecar anchors-validate`
- `sidecar anchors-rebind --threshold ... --dry-run`
- `sidecar anchor-history <doc_uid>`

### 5) Diagnostics and migration workflows

- `sidecar unresolved`
- `sidecar coverage` (documentation coverage, not test coverage)
- `sidecar check`
- `sidecar uid-migrations [--json]`

### 6) Future extension workflows

- `doc-suggest`
- `doc-generate` (AI-assisted, constrained)
- `graph-export`
- `stats`
- `semantic-search`

## Source of Truth

- Current CLI implementation: `crates/sidecar-cli/src/main.rs`
- Planned command model: `docs/CLI-SPEC.md`
- Docs export roadmap: `docs/SIDECAR-MKDOCS-EXPORTER-PLAN.md`

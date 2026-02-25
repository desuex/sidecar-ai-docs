# Sidecar to MkDocs Exporter Plan

## Goal

Build a deterministic exporter that converts Sidecar documentation (`docs-sidecar/` + index metadata) into MkDocs-ready Markdown pages under `docs/generated/`.

## Why

- Keep Sidecar as the source of truth.
- Publish docs through Read the Docs with MkDocs now.
- Avoid manual duplication between `docs-sidecar/` and published docs.

## Scope

Initial scope:

- Export symbol-bound sidecar docs to stable MkDocs pages.
- Generate deterministic navigation artifacts.
- Keep all output repo-relative and path-stable.

Out of scope (initial):

- Rendering every internal Sidecar field.
- Full-text search customization.
- Multi-language localization.

## Output Contract

Exporter output directory: `docs/generated/`

Required files:

- `docs/generated/README.md` (landing page for generated content)
- `docs/generated/symbols/<doc_uid>.md` (one page per sidecar doc)
- `docs/generated/_manifest.json` (deterministic manifest of generated pages)

Determinism requirements:

- Stable sorting by `doc_uid`, then path.
- Canonical JSON serialization in `_manifest.json`.
- No machine-specific absolute paths or timestamps in output content.

## Phased Plan

1. Phase 0: Bootstrap (completed in this change)
   - Add MkDocs and RTD config.
   - Add pre-build exporter hook script.
   - Add placeholder generated content.

2. Phase 1: Copy-through exporter (completed)
   - Parse `docs-sidecar/**/*.md`.
   - Copy to `docs/generated/symbols/` with deterministic naming.
   - Emit `_manifest.json`.

3. Phase 2: Structured rendering (completed)
   - Parse front matter.
   - Render stable title, summary, anchors section.
   - Add source links back to `docs-sidecar`.

4. Phase 3: Sidecar index integration (completed)
   - Read `.sidecar/index.sqlite` (or JSON export later).
   - Validate `symbol_uid` existence and flag unresolved anchors.
   - Generate unresolved report page.

5. Phase 4: CLI integration (completed)
   - Add `sidecar export mkdocs` command.
   - Make RTD pre-build call the CLI command instead of shell script.

## Execution Model

Current:

- RTD pre-build runs `sidecar export mkdocs --root . --out docs/generated` (via `cargo run --bin sidecar -- export mkdocs ...`).

## Testing Strategy

- Unit tests for path normalization and naming.
- Snapshot tests for generated page content.
- Snapshot tests for `_manifest.json`.
- CI smoke test: run exporter, then `mkdocs build --strict`.

## Risks and Mitigations

- Risk: nondeterministic ordering.
  - Mitigation: explicit sort before all writes.

- Risk: invalid/missing front matter.
  - Mitigation: fail with explicit error and file path.

- Risk: broken nav links.
  - Mitigation: generate and validate `_manifest.json` + strict MkDocs build.

## Definition of Done (MVP)

- `mkdocs build --strict` passes in CI.
- Exporter output is deterministic across repeated runs.
- At least one generated symbol page is snapshot-tested.

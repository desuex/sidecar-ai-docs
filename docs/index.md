# Sidecar AI Code Documentation

This site is built with MkDocs and published on Read the Docs.

Use this site for architecture, data model, indexing, storage, CLI, and MCP specs.

## Navigation

- Vision and principles: start with `Manifesto` and `Vision`.
- Technical design: see `Architecture`, `Data Model`, and `Anchoring`.
- Interfaces: see `CLI Spec` and `MCP Spec`.
- Validation: see `Evaluation Plan` and `Test Vectors`.

## Sidecar Export Track

The plan for a custom `sidecar+md -> MkDocs` exporter is tracked in:

- `SIDECAR-MKDOCS-EXPORTER-PLAN.md`

Generated content is written under `docs/generated/` during docs builds via:

- `sidecar export mkdocs --root . --out docs/generated`

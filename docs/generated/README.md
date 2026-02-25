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

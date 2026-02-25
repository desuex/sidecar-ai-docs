# Sidecar AI Code Documentation

[![codecov](https://codecov.io/gh/desuex/sidecar-ai-docs/graph/badge.svg?token=H5O3S2563V)](https://codecov.io/gh/desuex/sidecar-ai-docs)
[![docs](https://readthedocs.org/projects/docsai/badge/?version=latest)](https://docsai.readthedocs.io/)

## Intelligent, Refactor-Resistant Documentation Substrate for Codebases

---

## What This Is

Sidecar AI Code Documentation is an infrastructure layer for structured, queryable, refactor-resistant documentation attached to source code.

It is not a static site generator.

It is not a comment extractor.

It is not an API portal.

It is a semantic documentation substrate designed for:

* Large evolving codebases
* AI-assisted development
* Token-efficient LLM workflows
* Cross-reference-heavy systems
* Long-lived software projects

---

## Core Idea

Every meaningful element in a codebase should be:

* Identifiable (UID-first)
* Indexable (AST + semantic)
* Cross-referenceable
* Explainable
* Refactor-resilient
* Queryable by humans and AI

Documentation must live alongside code — but not inside it.

---

## Design Principles

* Documentation is structured data.
* Code identity precedes rendering.
* Anchors must survive refactors.
* Indexing is mandatory.
* Rendering is optional.
* AI must query structure — not raw files.
* Token economy is a first-class constraint.

---

## Architecture Overview

```
Codebase
   ↓
Tree-sitter + LSP
   ↓
Symbol Index (UID-first)
   ↓
Anchoring Layer
   ↓
Sidecar Doc Store
   ↓
Query Engine
   ↓
CLI / MCP / IDE / Export
```

---

## Key Capabilities

* Project-wide symbol reference resolution
* UID-based cross-referencing
* Refactor-resistant anchoring
* Semantic + structural indexing
* Offline index support (LSIF/SCIP)
* CLI interface
* MCP server for AI agents
* MCP doc coverage tools (`coverage_metrics`, `detect_undocumented_symbols`)
* VS Code extension
* JetBrains plugin
* Structured token-minimal responses

---

## Why This Exists

Modern codebases are too complex to navigate manually.

Developers should not need to:

* Grep blindly
* Reconstruct context
* Rely on tribal knowledge
* Load entire repositories into LLM context windows

This system exists to make code explainable without flooding context.

---

## What This Is Not

* Not a documentation website generator.
* Not a markdown renderer.
* Not a replacement for Sphinx or DocFX.
* Not an LLM wrapper over raw files.

Rendering layers may be added — but they are not the core.

---

## Long-Term Vision

* Every symbol explainable in one query.
* Architectural decisions traceable.
* Documentation that survives refactors.
* AI agents operating over structured code graphs.
* Minimal token usage.
* Maximum semantic precision.

---

## Repository Structure

See the `docs/` folder for full specifications:

* [Usage Scenarios](USAGE.md)
* [Manifesto & Vision](docs/MANIFESTO.md)
* [Architecture Overview](docs/ARCHITECTURE-OVERVIEW.md)
* [Data Model](docs/DATA-MODEL.md)
* [Indexing Specification](docs/INDEXING-SPEC.md)
* [UID and Cross-Reference Model](docs/UID-AND-XREF-MODEL.md)
* [Anchoring Specification](docs/ANCHORING-SPEC.md)
* [Storage Specification](docs/STORAGE-SPEC.md)
* [MCP Server Design](docs/MCP-SERVER-DESIGN.md)
* [CLI Specification](docs/CLI-SPEC.md)
* [IDE Integration](docs/IDE-INTEGRATION.md)
* [Token Economy Strategy](docs/TOKEN-ECONOMY-STRATEGY.md)
* [Evaluation Plan](docs/EVALUATION-PLAN.md)
* [Terminology & Glossary](docs/GLOSSARY.md)

---

## Status

Seed documentation phase.

Core specifications are being formalized prior to implementation.

---

## Documentation Build (MkDocs + RTD)

Read the Docs is configured with MkDocs using:

* `.readthedocs.yaml`
* `mkdocs.yml`

Local workflow:

1. Run exporter:
   * `cargo run --bin sidecar -- export mkdocs --root . --out docs/generated`
2. Install docs deps:
   * `pip install -r docs/requirements.txt`
3. Build docs:
   * `mkdocs build --strict`

Exporter implementation plan:

* `docs/SIDECAR-MKDOCS-EXPORTER-PLAN.md`
* Current exporter output:
  * `docs/generated/_manifest.json`
  * `docs/generated/symbols/<doc_uid>.md`
  * `docs/generated/reports/unresolved-anchors.md`

---

## Test Coverage (Codecov)

Coverage reporting is integrated with Codecov and gated at **90% line coverage**.

Local coverage command:

* `./scripts/ci/coverage.sh`

The script:

* generates `lcov.info` with `cargo llvm-cov`
* computes line coverage from LCOV data
* fails if coverage drops below `90%` (configurable via `MIN_COVERAGE`)

Documentation coverage non-regression is also gated in CI using MCP:

* `./scripts/ci/doc_coverage_gate.sh`
* baseline config: `scripts/ci/doc_coverage_baseline.env`

---

## License

See LICENSE file.

---

## Contributing

See CONTRIBUTING.md.

---

## Security

See SECURITY.md.

---

## Governance

See GOVERNANCE.md.

---

Sidecar AI Code Documentation
A substrate for explainable code.

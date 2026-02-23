# Sidecar AI Code Documentation

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

See `docs/` for full specifications:

* Architecture
* Data model
* Indexing
* Anchoring
* MCP
* CLI
* IDE integration
* Token economy
* Evaluation model

---

## Status

Seed documentation phase.

Core specifications are being formalized prior to implementation.

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

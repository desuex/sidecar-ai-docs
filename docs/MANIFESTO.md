# docs/MANIFESTO.md

---

# Sidecar AI Code Documentation

## Manifesto

---

## 1. Why This Exists

Modern codebases are not short.

They are not linear.

They are not fully understood by any single human.

And yet, the primary documentation model we use is still:

* Inline comments
* Static site generators
* Markdown pages disconnected from code
* Human memory

We rely on developers remembering context, reconstructing intent, and manually searching across thousands of files.

This is inefficient.

This is cognitively expensive.

This does not scale.

---

## 2. The Core Problem

When working on a piece of code, a developer should be able to answer instantly:

* What does this symbol actually do?
* Where is it used?
* Why does it exist?
* What architectural decision led to it?
* What will break if I change it?
* What documentation refers to it?
* What other systems depend on it?

Today, answering those questions requires:

* IDE navigation
* Grep
* Guesswork
* Tribal knowledge
* Context reconstruction

We want to reduce that to:

> One command. One query. One hover.

---

## 3. Documentation Should Be First-Class Data

Documentation is not:

* A rendered website.
* A side effect of code comments.
* A static artifact.

Documentation is structured knowledge attached to program structure.

It must be:

* Queryable
* Cross-referenced
* Refactor-resistant
* Version-aware
* Machine-readable
* Human-readable
* Agent-accessible

---

## 4. The Sidecar Principle

Documentation must not live only inside source files.

Inline comments are fragile:

* They move.
* They rot.
* They are rarely structured.
* They are hard to analyze at scale.

Instead, documentation should live in a **sidecar model**:

* Stored separately from code
* Anchored to symbols and structures
* Survives refactors
* Version-controlled
* Indexable

Code is the executable graph.
Documentation is the semantic graph.

They must be linked — but not conflated.

---

## 5. AI Agents Are Not Replacements — They Are Multipliers

AI does not replace documentation.

AI amplifies documentation when:

* It can query structure precisely.
* It can access stable symbol identities.
* It can retrieve only relevant slices.
* It can reason over structured indexes instead of raw files.

This system is not "LLM reads entire repo."

This system is:

> LLM asks structured questions to a code-intelligence substrate.

Token usage must be minimized.

Context windows must not be flooded.

Agents must operate on indexed knowledge — not raw text dumps.

---

## 6. Refactor-Resistant Knowledge

Line numbers are not anchors.

Files are not stable.

Symbols move.

Functions split.

Classes merge.

Documentation must survive:

* Renames
* Moves
* Reordering
* Partial rewrites
* Extract-method refactors

Anchoring must combine:

* Semantic identity (symbol UID)
* Structural identity (AST path + fingerprint)
* Fuzzy reattachment (context-based selectors)
* Rebase strategies (AST diff)

This is not optional.

Without refactor resistance, documentation becomes noise.

---

## 7. UID-First Design

Everything important must have a stable identity:

* Symbols
* Modules
* Files
* Concepts
* Architectural components
* Documentation units

Identity precedes rendering.

Identity precedes UI.

Identity precedes markdown.

Cross-references must resolve via UID, not text matching.

---

## 8. One Substrate, Many Interfaces

The system must expose:

* CLI for humans
* MCP server for AI agents
* VS Code extension
* JetBrains plugin
* CI index export

The core must be:

* Editor-agnostic
* Language-agnostic
* UI-agnostic

One index.

One doc store.

Many clients.

---

## 9. Token Economy Is a Design Constraint

LLM context is expensive.

Human attention is expensive.

The system must:

* Return only requested fields
* Limit snippet size
* Support structured queries
* Support ranking
* Avoid dumping entire files

The goal is not verbosity.

The goal is precision.

---

## 10. This Is Not a Documentation Generator

This project does not aim to:

* Replace Sphinx.
* Replace Doxygen.
* Replace DocFX.
* Generate static sites by default.

Those are rendering layers.

This project is:

> A documentation substrate for intelligent systems.

Rendering is optional.

Indexing is mandatory.

Anchoring is mandatory.

Identity is mandatory.

---

## 11. Long-Term Vision

The long-term outcome is:

* A codebase where every symbol is explainable.
* A system where architectural decisions are queryable.
* An environment where documentation evolves with code.
* An AI agent that never needs the whole repository to answer questions.

Developers should think less about:

* "Where was that defined?"
* "Who uses this?"
* "What does this break?"

And more about:

* "What should this become?"

---

## 12. Principle Summary

We believe:

* Documentation is structured knowledge.
* Structure must be indexable.
* Index must be queryable.
* Identity must be stable.
* Anchors must survive change.
* Interfaces must be minimal and precise.
* AI must operate on structured substrate — not raw text.

This project exists to build that substrate.

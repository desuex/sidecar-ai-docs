Glossary

⸻

1. Purpose

This glossary defines core terminology used throughout the system.

Terms must be interpreted precisely.

Definitions are normative unless explicitly marked as descriptive.

⸻

2. Core Concepts

Anchor

A structured binding between documentation and a code entity.

Anchors are:
	•	Refactor-aware
	•	AST-based
	•	Confidence-scored
	•	Not line-number dependent

Anchors survive formatting and reordering.

⸻

Anchor Confidence

A numeric value (0.0–1.0) representing how strongly an anchor matches a symbol after changes.

Used to:
	•	Signal rebind quality
	•	Trigger warnings
	•	Avoid silent misbinding

⸻

Anchor Rebinding

The process of reattaching a documentation anchor to a modified symbol after refactor.

Uses:
	•	AST diff
	•	Similarity scoring
	•	Structural matching

⸻

AST (Abstract Syntax Tree)

Tree representation of source code structure.

Used for:
	•	Symbol extraction
	•	Fingerprinting
	•	Anchoring
	•	Diffing
	•	Structural identity

⸻

AST Diff

Structural comparison between two AST versions of a file.

Used to:
	•	Detect renames
	•	Detect moves
	•	Detect structural similarity
	•	Guide anchor rebinding

⸻

CLI

Command-line interface for interacting with the system.

Provides:
	•	Symbol queries
	•	Documentation queries
	•	Reference queries
	•	Index management

Must be deterministic and script-friendly.

⸻

Code Coverage (Documentation Coverage)

Percentage of public symbols with attached documentation.

Not related to test coverage.

⸻

Concept

A documentation entity that groups multiple symbols under a shared idea.

Example:
	•	“Authentication Flow”
	•	“Billing Architecture”

Concepts may bind to multiple anchors.

⸻

Determinism

Property that identical inputs produce identical outputs.

Applies to:
	•	UID generation
	•	Ranking
	•	Pagination
	•	MCP responses
	•	CLI JSON output

⸻

Fingerprint

Hash derived from AST structure of a symbol.

Used for:
	•	UID generation
	•	Similarity comparison
	•	Rename detection
	•	Refactor tracking

Must ignore formatting noise.

⸻

Incremental Indexing

Updating only changed files instead of full reindex.

Must be:
	•	Deterministic
	•	Bounded
	•	Safe under concurrent edits

⸻

Index

Structured database containing:
	•	Symbols
	•	UIDs
	•	References
	•	Anchors
	•	Documentation bindings
	•	Ranking metadata

Core persistent state.

⸻

LSIF

Language Server Index Format.

External format describing symbol graphs.

May be imported into system.

⸻

MCP (Model Context Protocol)

Structured protocol exposing index functionality to AI agents and tools.

Provides:
	•	Deterministic queries
	•	Structured JSON responses
	•	Progressive disclosure
	•	Token-efficient access

⸻

Migration Event

Record of UID change due to refactor.

Includes:
	•	Old UID
	•	New UID
	•	Reason (rename, move, split, etc.)
	•	Similarity score

⸻

Pagination Stability

Property that paginated results remain stable across repeated queries.

Required for deterministic behavior.

⸻

Ranking

Deterministic scoring system for ordering search results.

Based on:
	•	Lexical match
	•	Symbol visibility
	•	Reference frequency
	•	Structural similarity

⸻

Relevance Score

Numeric value (0–1) representing ranking strength.

Used for:
	•	Sorting
	•	Debugging
	•	Deterministic ordering

⸻

Refactor Resilience

Ability of documentation bindings to survive code changes.

Measured via:
	•	Anchor confidence
	•	Migration detection accuracy
	•	UID stability

⸻

Schema Version

Version number representing structure of index storage or MCP interface.

Used to:
	•	Detect incompatibility
	•	Trigger migrations
	•	Maintain backward compatibility

⸻

Selector

Structured AST-based descriptor used to attach documentation to non-symbol nodes.

Used for:
	•	Anonymous functions
	•	Specific code blocks
	•	Conditional statements
	•	Expression-level anchors

Selectors are:
	•	Structural
	•	Hash-assisted
	•	Confidence-scored

⸻

Sidecar

Documentation file stored separately from source code.

Contains:
	•	YAML metadata
	•	Anchor definitions
	•	Markdown content

Sidecar approach avoids modifying source code.

⸻

Similarity Score

Numeric measure used in AST diff and rebinding.

Combines:
	•	Node type similarity
	•	Subtree hash similarity
	•	Name similarity
	•	Parent context similarity

⸻

Symbol

A named code entity extracted from AST.

Examples:
	•	Class
	•	Function
	•	Method
	•	Interface
	•	Enum
	•	Module
	•	Variable (if public)

Each symbol has:
	•	UID
	•	Qualified name
	•	Kind
	•	Fingerprint
	•	Visibility

⸻

Token Economy

Architectural discipline to minimize token usage in AI interactions.

Principles:
	•	Structured responses
	•	Progressive disclosure
	•	Pagination
	•	Field-level selection
	•	No implicit expansion

⸻

UID (Unique Identifier)

Deterministic identifier for a symbol.

Derived from:
	•	Language
	•	Module path
	•	Qualified name
	•	Structural fingerprint

Must be:
	•	Stable under formatting
	•	Refactor-aware
	•	Versioned
	•	Collision-resistant

⸻

UID Determinism

Property that identical symbol structure produces identical UID.

Required for:
	•	Stable documentation binding
	•	Predictable behavior
	•	CI reproducibility

⸻

Unresolved Anchor

Anchor that cannot bind to any symbol after refactor.

Must:
	•	Be reported
	•	Not silently rebound
	•	Preserve documentation content

⸻

Versioned Ranking

Ranking algorithm version tracked explicitly.

Changes must:
	•	Be versioned
	•	Preserve determinism
	•	Not silently reorder results

⸻

3. Meta-Concepts

Thin Client

IDE plugin or external tool that delegates all logic to MCP + index.

Must not:
	•	Access storage directly
	•	Reimplement parsing logic

⸻

Progressive Disclosure

Pattern of returning minimal metadata first and expanding only when requested.

Reduces token usage.

⸻

Structured Response

JSON-based response containing explicit fields rather than verbose prose.

Enables:
	•	Determinism
	•	Token efficiency
	•	Machine reasoning

⸻

Bounded Query

Query with explicit limits:
	•	limit
	•	offset
	•	depth
	•	max_tokens

Prevents unbounded graph expansion.

⸻

Stability Contract

Guarantee that certain properties (UID, ranking, schema) remain stable across minor versions.

Breaking changes require major version bump.

⸻

4. Non-Terms (Clarifications)

The system does NOT:
	•	Execute code
	•	Infer business meaning
	•	Replace LSP
	•	Replace version control
	•	Replace documentation sites
	•	Learn dynamically from usage

It provides structured, deterministic access to code structure and documentation.

⸻

5. Summary

This glossary defines:
	•	Identity concepts (UID, fingerprint)
	•	Binding concepts (anchor, selector)
	•	Interaction concepts (MCP, CLI)
	•	Determinism concepts (ranking, pagination)
	•	Security concepts (untrusted input)
	•	Architectural constraints (token economy)

Terminology must be used consistently.

Precision is required.

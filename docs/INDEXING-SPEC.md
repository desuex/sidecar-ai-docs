# Indexing Specification

---

## 1. Purpose

This document defines the requirements, data structures, and lifecycle of the indexing subsystem. Indexing serves as the primary foundation of the Sidecar AI Code Documentation system. Without a persistent, language-aware index, documentation cannot be successfully cross-referenced, made refactor-resilient, efficiently queried, or rendered AI-compatible. Therefore, the indexing process is a mandatory component of the architecture.

---

## 2. Goals

The indexing subsystem must:

* Parse code into structured representation (AST)
* Extract symbol definitions
* Extract references
* Assign stable UIDs
* Build a project-wide symbol graph
* Support incremental updates
* Persist index state
* Expose structured query interface

It must not:

* Render documentation
* Store UI-specific data
* Embed markdown rendering logic

---

## 3. Core Responsibilities

The index must track:

### 3.1 Symbols

For each symbol:

* UID
* Name
* Kind (function, class, variable, module, etc.)
* Language
* File path
* Range (start/end)
* Signature (if applicable)
* Parent UID
* Visibility (public/private/internal)
* Export status

---

### 3.2 References

For each reference:

* Source symbol UID
* Target symbol UID
* File
* Range
* Context type:

  * Call
  * Import
  * Type usage
  * Inheritance
  * Field access
  * Instantiation

---

### 3.3 File Metadata

For each file:

* File UID
* Language
* Hash (content fingerprint)
* Dependency list
* Defined symbols
* Imported symbols

---

### 3.4 Module Graph

Track:

* Module boundaries
* Import relationships
* Package structure
* Dependency graph edges

---

## 4. Index Data Model

*(For full details, refer to the [Data Model Specification](DATA-MODEL.md))*

The index operates as a graph data structure representing code elements and their relationships.

### 4.1 Symbol Node

```json
{
  "uid": "sym:lang:path:qualified_name:hash",
  "name": "MyFunction",
  "kind": "function",
  "file_uid": "file:src/myfile.ts",
  "parent_uid": "sym:lang:path:MyClass",
  "signature": "(a: number) => string",
  "visibility": "public"
}
```

---

### 4.2 Reference Edge

```json
{
  "from_uid": "sym:caller",
  "to_uid": "sym:callee",
  "type": "call",
  "file_uid": "file:src/usage.ts",
  "range": { "start": 120, "end": 135 }
}
```

---

### 4.3 File Node

```json
{
  "uid": "file:src/utils/math.ts",
  "language": "typescript",
  "hash": "sha256:abcdef",
  "defined_symbols": ["sym:..."],
  "imports": ["sym:..."]
}
```

---

## 5. UID Requirements

*(For complete identity rules, see the [UID and Cross-Reference Model](UID-AND-XREF-MODEL.md))*

The UID system provides stable identities. Each UID must be:

* Deterministic
* Stable across formatting changes
* Based on:

  * Qualified name
  * Structural position
  * Language identifier
* Resistant to:

  * Reordering of functions
  * Whitespace changes
  * Comment edits

UID must not rely on:

* Line numbers
* Byte offsets alone

---

## 6. Parsing Strategy

The system supports multiple parsing backends.

### 6.1 Tree-sitter

* Fast
* Incremental
* Multi-language
* AST-focused

Used for:

* Structural parsing
* Symbol extraction
* Range detection

---

### 6.2 LSP Integration

Optional integration for:

* Accurate semantic resolution
* Type resolution
* Cross-file symbol resolution

Used when available.

---

### 6.3 LSIF / SCIP Import

Optional:

* Import precomputed symbol indexes
* Support large repositories
* Reuse CI-generated indexes

---

## 7. Incremental Indexing

Full re-indexing is unacceptable for large repositories.

The index must support:

* File-level incremental updates
* AST diffing
* Symbol diffing
* Reference recalculation for affected nodes

Workflow:

1. Detect file change.
2. Reparse file.
3. Compare old vs new AST.
4. Update affected symbols.
5. Recompute affected references.
6. Update graph edges.
7. Preserve UIDs where possible.

---

## 8. Persistence Layer

The index must persist across sessions.

Storage options:

* SQLite
* RocksDB
* Custom binary store
* JSON for debugging

Requirements:

* Fast lookup by UID
* Reverse reference lookup
* Range-based lookup
* Graph traversal support
* Partial loading

Persistence format must be versioned.

---

## 9. Query Capabilities

The index must support:

### 9.1 Direct Lookup

* get_symbol(uid)
* get_file(uid)

---

### 9.2 Reverse References

* find_references(uid)
* find_callers(uid)
* find_dependents(uid)

---

### 9.3 Structural Traversal

* get_children(uid)
* get_parent(uid)
* get_module_graph()

---

### 9.4 Impact Analysis

* what_breaks_if_changed(uid)
* transitive_dependents(uid)

---

## 10. Ranking Layer

Index must provide metadata for ranking:

* Reference frequency
* Call depth
* Public exposure
* Graph centrality
* Module boundaries

Ranking is consumed by query engine.

Index does not generate summaries.

---

## 11. Performance Requirements

Index must:

* Handle large repositories (>1M LOC)
* Support sub-second symbol lookup
* Support sub-second reference queries
* Avoid full repository scanning per query

---

## 12. Language Agnosticism

The core index must:

* Normalize symbol model across languages
* Abstract language-specific details
* Store language metadata separately

Language adapters must translate AST → normalized model.

---

## 13. Error Handling

Index must:

* Tolerate parse errors
* Store partial symbol data
* Flag incomplete resolution
* Avoid index corruption

No silent failure.

---

## 14. Versioning

Index schema must be versioned.

On schema change:

* Migrate automatically
  or
* Invalidate and rebuild deterministically

---

## 15. Non-Goals

Index is not:

* A renderer
* A markdown processor
* A search engine for plain text
* An embedding store (optional extension)
* A code formatter

Index is a semantic symbol graph.

---

## 16. Security Considerations

Index must:

* Not execute code
* Avoid arbitrary plugin execution
* Sanitize file paths
* Protect against malicious repositories

---

## 17. Future Extensions

* Embedding-based semantic search
* Graph-based anomaly detection
* Architecture cluster detection
* Dead symbol detection
* Change risk scoring

These extend the index but do not redefine it.

---

## 18. Summary

Indexing is:

* The backbone of the system.
* The foundation for refactor resilience.
* The prerequisite for token-efficient AI queries.
* The core abstraction layer between code and documentation.

Without index integrity, nothing else works.

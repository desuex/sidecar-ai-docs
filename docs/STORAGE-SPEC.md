# Storage Specification

---

## 1. Purpose

This document defines how data is stored in the system.

Storage must support:

* Persistent indexing
* UID-based lookup
* Cross-reference traversal
* Anchor validation
* Incremental updates
* Versioning
* Deterministic rebuild

Storage is an implementation detail.

The logical model is canonical.

---

## 2. Storage Layers

The system stores:

1. Symbol Index
2. Reference Graph
3. File Metadata
4. Documentation Store
5. Anchor Metadata
6. UID Migration Log
7. Schema Versioning Data

Each layer must be independently queryable.

---

## 3. Storage Design Principles

Storage must be:

* Durable
* Fast for UID lookup
* Fast for reverse reference lookup
* Incrementally updatable
* Versioned
* Portable
* Deterministic
* Safe for large repositories

Storage must not:

* Store entire file contents unnecessarily
* Store rendered documentation
* Store transient LSP state
* Store UI-specific metadata

---

## 4. Physical Storage Options

Allowed storage backends:

* SQLite (recommended default)
* RocksDB
* LMDB
* Key-value store
* Custom binary store
* JSON (debug mode only)

Logical schema must be independent of backend.

---

## 5. Logical Schema Overview

Core logical tables:

* symbols
* references
* files
* modules
* documentation_units
* anchors
* concepts
* uid_migrations
* schema_metadata

---

## 6. Symbol Storage

### Table: symbols

Fields:

* uid (primary key)
* name
* qualified_name
* kind
* language
* file_uid
* parent_uid
* visibility
* is_exported
* signature
* fingerprint
* created_at
* updated_at

Indexes required:

* uid
* file_uid
* parent_uid
* qualified_name

---

## 7. Reference Storage

### Table: references

Fields:

* from_uid
* to_uid
* type
* file_uid
* range_start
* range_end

Indexes required:

* from_uid
* to_uid
* file_uid

Reverse lookup must be optimized.

---

## 8. File Storage

### Table: files

Fields:

* uid
* path
* language
* hash
* last_indexed_at

Indexes required:

* uid
* path

Hash used for incremental re-index.

---

## 9. Documentation Storage

### Table: documentation_units

Fields:

* doc_uid
* title
* content (plain text or markdown)
* created_at
* updated_at
* version

Content must be stored as plain text.

Binary content not allowed.

---

## 10. Anchor Storage

### Table: anchors

Fields:

* doc_uid
* anchor_type
* symbol_uid (nullable)
* selector_json (nullable)
* fingerprint
* confidence
* last_verified_at

Indexes required:

* doc_uid
* symbol_uid

Selector JSON must be canonicalized.

---

## 11. Concept Storage

### Table: concepts

Fields:

* uid
* name
* description

Cross-table relationships:

* concept_symbol_links
* concept_doc_links

---

## 12. UID Migration Log

### Table: uid_migrations

Fields:

* old_uid
* new_uid
* similarity_score
* reason
* timestamp

Migration log must be append-only.

Used for audit and traceability.

---

## 13. Schema Versioning

### Table: schema_metadata

Fields:

* schema_version
* index_version
* created_at
* last_migration_at

On schema change:

* Migrate automatically
  or
* Invalidate index and rebuild

Version mismatch must not corrupt data.

---

## 14. Incremental Update Strategy

On file change:

1. Compare file hash.
2. If unchanged → skip.
3. If changed:

   * Reparse AST.
   * Diff symbols.
   * Update symbol table.
   * Update reference table.
   * Run anchor validation.

Must avoid full index rebuild.

---

## 15. Query Requirements

Storage must support:

* get_symbol(uid)
* find_references(uid)
* get_documentation(uid)
* get_anchors(uid)
* impact_analysis(uid)
* reverse_concept_lookup(uid)

Queries must:

* Support field filtering
* Support pagination
* Support ordering
* Avoid full table scan

---

## 16. Performance Constraints

Storage must:

* Handle >1M symbols
* Support sub-second lookup
* Support high reference counts
* Avoid locking entire DB during incremental update
* Scale with repository size

---

## 17. File System Layout

Sidecar layout inside repository:

```text
.project_root/
  .sidecar/
    index.db
    metadata.json
    version.json
```

Documentation store may be:

```text
.project_root/
  docs-sidecar/
    doc:cart-calc-overview.md
    doc:pricing-engine.md
```

Or stored entirely in database.

Hybrid model allowed.

---

## 18. Sidecar Separation Rules

Documentation must not:

* Modify source files
* Inject comments automatically
* Depend on inline doc blocks

Sidecar folder must:

* Be git-versionable
* Be human-readable
* Be portable across environments

---

## 19. Backup and Recovery

Storage must support:

* Full rebuild from source
* Deterministic regeneration
* Export/import
* Snapshotting

Index must be reproducible.

Documentation must survive index rebuild.

---

## 20. Concurrency Model

Storage must support:

* Read-heavy workloads
* Single-writer indexing
* Safe concurrent reads
* Transaction boundaries for updates

Deadlocks must be prevented.

---

## 21. Security Considerations

Storage must:

* Sanitize inputs
* Prevent injection via malformed UIDs
* Avoid path traversal
* Validate JSON fields
* Limit document size

Database must not execute arbitrary code.

---

## 22. Extensibility

Storage must allow:

* Additional metadata fields
* Additional reference types
* Embedding storage (optional)
* Index augmentation

Extensibility must preserve schema integrity.

---

## 23. Non-Goals

Storage does not:

* Store runtime state
* Store LLM prompt history
* Store rendered HTML
* Store entire source file text
* Replace version control

---

## 24. Summary

Storage is:

* The durable backbone of the system.
* The persistence layer for identity and graph.
* The audit trail of refactor evolution.
* The foundation for query efficiency.

Logical model defines meaning.

Storage implements durability.

Both must remain consistent.

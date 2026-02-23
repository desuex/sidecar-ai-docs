---
doc_uid: doc:repository-trait
title: Repository — Storage Abstraction Trait
anchors:
  - anchor_type: symbol
    symbol_uid: sym:rs:crates/sidecar-core/src/repository:Repository:f3ba5576
    confidence: 1.0
---

## Overview

`Repository` is the core storage abstraction trait. It defines the contract between the indexing/query layer (`sidecar-core`) and the storage backend (`sidecar-storage`). This dependency inversion allows the core logic to remain storage-agnostic.

## Methods

- `upsert_file` / `upsert_symbols` / `upsert_refs` / `upsert_docs` — write operations
- `get_file_by_path` — incremental indexing lookup
- `search_symbols` — paginated symbol search with deterministic ordering
- `get_symbol` — single symbol lookup by UID
- `find_refs` — reference query (M2)
- `get_doc` — documentation lookup by target symbol UID

## Design Decisions

- No `Send + Sync` bounds — `rusqlite::Connection` is not `Sync`, and each thread should own its own repository instance
- All methods take `&self` — interior mutability handled by the implementation
- Returns `Result<_, SidecarError>` for uniform error handling with exit codes

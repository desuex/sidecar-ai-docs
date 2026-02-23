---
doc_uid: doc:index-project
title: index_project — Indexing Pipeline
anchors:
  - anchor_type: symbol
    symbol_uid: sym:rs:crates/sidecar-core/src/indexer:index_project:7e7f9eb9
    confidence: 1.0
---

## Overview

`index_project` is the main indexing entry point. It walks a project directory, identifies supported source files by extension, parses them with the appropriate `LanguageAdapter`, generates deterministic UIDs via BLAKE3 fingerprints, and stores the results via the `Repository` trait. It supports incremental indexing by comparing content hashes.

## Pipeline

1. Walk directory with `walkdir`, skip hidden dirs and `node_modules`/`target`/etc
2. Sort entries by path for deterministic processing order
3. For each supported file: compute content hash, skip if unchanged
4. Parse symbols via the matching `LanguageAdapter`
5. Generate UID per symbol: `sym:<lang>:<module_path>:<qualified_name>:<fingerprint_prefix>`
6. Store via `repo.upsert_file()` + `repo.upsert_symbols()`

## Returns

`IndexResult { files_indexed, files_skipped, symbols_extracted, duration_ms }`

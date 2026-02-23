---
doc_uid: doc:sqlite-repository
title: SqliteRepository — SQLite Storage Backend
anchors:
  - anchor_type: symbol
    symbol_uid: sym:rs:crates/sidecar-storage/src/sqlite:SqliteRepository:74b8c482
    confidence: 1.0
---

## Overview

`SqliteRepository` implements the `Repository` trait using SQLite via `rusqlite` (bundled). It stores all indexed data in a single `.sidecar/index.sqlite` file. The implementation uses WAL mode for read performance and transactions for atomic symbol batch writes.

## Key Methods

- `open(path)` — opens or creates the database, runs migrations, enables WAL
- `open_in_memory()` — creates an in-memory database for testing
- `upsert_symbols` — transactional: deletes old symbols for the file, then inserts new batch
- `search_symbols` — LIKE query with deterministic ORDER BY (name ASC, uid ASC)
- `get_doc` — looks up documentation by target symbol UID

## Schema

Five tables: `meta`, `files`, `symbols`, `refs`, `docs` — with indexes on frequently queried columns.

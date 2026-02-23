---
doc_uid: doc:uid-type
title: Uid — Validated Symbol Identity
anchors:
  - anchor_type: symbol
    symbol_uid: sym:rs:crates/sidecar-types/src/uid:Uid:a5036a51
    confidence: 1.0
---

## Overview

`Uid` is a validated newtype wrapping a `String` that enforces the Sidecar UID format. All identity in the system flows through UIDs. They are deterministic, stable across formatting changes, and validated at parse time to reject path traversal, spaces, and unknown prefixes.

## Format

UIDs follow the pattern `<prefix>:<segments>`:

- `sym:<lang>:<module_path>:<qualified_name>:<struct_hash>` — code symbols
- `file:<path>` — indexed files
- `module:<path>` — module identifiers
- `doc:<slug>` — documentation entries
- `concept:<slug>` — concept identifiers

The `struct_hash` is the first 8 hex characters of a BLAKE3 fingerprint, ensuring UIDs survive formatting-only edits.

## Design Decisions

- Validated at parse time (`FromStr`) — invalid UIDs cannot exist in the type system
- `Ord` derived for deterministic ordering in `BTreeMap` usage
- Serde transparent — serializes as plain string in JSON

---
doc_uid: doc:language-adapter
title: LanguageAdapter — Language Parsing Contract
anchors:
  - anchor_type: symbol
    symbol_uid: sym:rs:crates/sidecar-parsing/src/adapter:LanguageAdapter:efe7ed94
    confidence: 1.0
---

## Overview

`LanguageAdapter` is the trait that all language parsers must implement. It provides two methods: `parse_symbols` for extracting symbol definitions from source code, and `parse_refs` for extracting references between symbols. Each adapter is associated with a specific `Language` variant.

## Contract

- `language()` — returns which `Language` this adapter handles
- `parse_symbols(source: &[u8]) -> Vec<RawSymbol>` — extract symbol definitions, sorted by range start for determinism
- `parse_refs(source: &[u8]) -> Vec<RawRef>` — extract references (stub in M1, implemented in M2)

## Implementations

- `TypeScriptAdapter` — TypeScript/JavaScript via `tree-sitter-typescript`
- `RustAdapter` — Rust via `tree-sitter-rust`

## Design Decisions

- Takes `&[u8]` not `&str` — tree-sitter works with byte slices
- Returns `Vec` not iterator — simplifies deterministic sorting
- Uses `RefCell<Parser>` internally because tree-sitter `parse()` needs `&mut self` but the trait uses `&self`

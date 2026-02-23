---
doc_uid: doc:rust-adapter
title: RustAdapter — Rust Language Parser
anchors:
  - anchor_type: symbol
    symbol_uid: sym:rs:crates/sidecar-parsing/src/rust:RustAdapter:33d772a2
    confidence: 1.0
---

## Overview

`RustAdapter` implements `LanguageAdapter` for Rust source files using `tree-sitter-rust`. It extracts structs (as Class), enums, traits (as Interface), functions, methods (from `impl` blocks with qualified names like `TypeName.method_name`), type aliases, constants, statics, and modules. Visibility is determined by the presence of a `visibility_modifier` node (`pub`).

## Symbol Mapping

| Rust construct | SymbolKind | Qualified name |
|---|---|---|
| `fn foo()` | Function | `foo` |
| `struct Foo` | Class | `Foo` |
| `impl Foo { fn bar() }` | Method | `Foo.bar` |
| `trait Foo` | Interface | `Foo` |
| `enum Foo` | Enum | `Foo` |
| `type Foo = ...` | Type | `Foo` |
| `const FOO` | Constant | `FOO` |
| `mod foo` | Module | `foo` |

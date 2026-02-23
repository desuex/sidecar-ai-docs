# Tree-sitter Specification

---

## 1. Purpose

This document defines how Tree-sitter is used within the system.

Tree-sitter is the primary structural parsing engine.

It is responsible for:

* Incremental parsing
* AST construction
* Structural symbol extraction
* Range tracking
* Structural fingerprinting for anchoring

Tree-sitter does not:

* Perform semantic type resolution
* Maintain persistent symbol graph
* Store documentation

It is a parsing substrate only.

---

## 2. Why Tree-sitter

Tree-sitter provides:

* Fast incremental parsing
* Concrete syntax tree (CST) + AST-like traversal
* Language-agnostic parsing model
* Robust multi-language support
* Node-level range information
* Stable tree diffing capability

It enables:

* Structural analysis without full compiler
* Incremental updates
* Precise anchor binding

---

## 3. Role in System Architecture

Tree-sitter operates in the Parsing Layer.

```text
Source File
   ↓
Tree-sitter Parser
   ↓
Syntax Tree
   ↓
Symbol Extractor
   ↓
Normalized Symbol Model
```

Output flows into the Indexing Layer.

---

## 4. Language Adapter Model

Each supported language must implement:

* Grammar binding
* Symbol extraction rules
* Node kind mapping
* Visibility detection
* Import detection
* Export detection

The adapter translates Tree-sitter nodes into:

* Symbol definitions
* Reference records
* Structural fingerprints

Adapters must remain isolated per language.

---

## 5. Symbol Extraction Strategy

Symbol extraction must:

1. Identify top-level constructs.
2. Traverse nested scopes.
3. Extract:

   * Name
   * Kind
   * Parent
   * Visibility
   * Signature (if available)
   * Range
4. Compute deterministic UID input.

Examples of symbol kinds:

* Module
* Class
* Struct
* Interface
* Function
* Method
* Variable
* Constant
* Enum
* Type alias
* Field
* Property

---

## 6. Reference Extraction Strategy

References are identified via:

* Identifier nodes
* Call expressions
* Member expressions
* Type references
* Import statements
* Inheritance declarations

Reference extraction must:

* Capture textual occurrence
* Record context type
* Record enclosing symbol
* Provide range

Resolution of reference → target UID may be:

* Deferred to LSP layer
* Or handled via heuristic matching

Tree-sitter alone cannot resolve types in many languages.

---

## 7. Incremental Parsing

Tree-sitter supports incremental parsing via:

* Tree reuse
* Change ranges
* Partial reparse

Workflow:

1. Detect file modification.
2. Apply incremental edit.
3. Reparse only changed region.
4. Compare old and new trees.
5. Extract changed symbols.
6. Emit symbol diff.

This enables efficient re-indexing.

---

## 8. Structural Fingerprinting

Tree-sitter enables anchor resilience via:

* Node type
* Parent chain
* Sibling ordering
* Subtree hash

Structural fingerprint example:

```text
function_declaration
  parent: class_declaration
  name: calculate
  parameters: [number, number]
  depth: 3
```

Fingerprint must:

* Exclude whitespace
* Exclude comments
* Include syntactic structure
* Include qualified name

Used for:

* UID stability
* Anchor rebasing
* Diff-based reattachment

---

## 9. Limitations of Tree-sitter

Tree-sitter does not:

* Perform semantic resolution
* Understand types fully
* Detect dynamic dispatch
* Resolve cross-file imports reliably
* Execute language-specific logic

Therefore:

Tree-sitter provides structure.
LSP may provide semantics.

---

## 10. Integration with LSP

When LSP is available:

* Use Tree-sitter for structure
* Use LSP for:

  * Definition resolution
  * Reference resolution
  * Type resolution

When LSP is unavailable:

* Fallback to heuristic resolution
* Provide partial index

System must degrade gracefully.

---

## 11. Error Handling

Tree-sitter can parse incomplete or invalid code.

Parser must:

* Accept error nodes
* Extract partial symbols when possible
* Flag incomplete nodes
* Avoid crashing index

Index must store parse error state per file.

---

## 12. Performance Requirements

Tree-sitter layer must:

* Parse single file under 50ms (typical size)
* Support large files gracefully
* Avoid full tree traversal when unnecessary
* Reuse node references for incremental diff

Parsing must not block UI thread in IDE context.

---

## 13. Multi-Language Support

Supported languages must:

* Define grammar dependency
* Implement symbol adapter
* Define kind mapping
* Define visibility rules

Core system must not hardcode language logic.

All language behavior must exist in adapters.

---

## 14. Determinism

Tree-sitter parsing must be deterministic.

UID input derived from Tree-sitter must:

* Produce identical result on identical code
* Not depend on runtime state
* Not depend on file order

Determinism is critical for stable indexing.

---

## 15. Testing Requirements

For each language adapter:

* Unit tests for symbol extraction
* Tests for nested scopes
* Tests for reference extraction
* Tests for incremental reparse behavior
* Tests for structural fingerprint consistency

Test vectors must include:

* Renames
* Function moves
* Extract method
* Reorder members
* Add/remove comments

---

## 16. Security Considerations

Tree-sitter layer must:

* Not execute arbitrary code
* Not evaluate macros
* Not run build tools
* Only parse text

All parsing must be sandboxed and safe.

---

## 17. Future Extensions

Possible enhancements:

* Tree-sitter query DSL integration
* Pattern-based symbol extraction
* Cross-language symbol mapping
* Code pattern detection
* Structural anomaly detection

These extend parsing capabilities but do not redefine its role.

---

## 18. Summary

Tree-sitter is:

* The structural parsing backbone.
* The provider of incremental AST.
* The source of structural fingerprints.
* The first step toward stable UIDs.
* A deterministic, language-agnostic parsing substrate.

It provides structure.

Structure enables identity.

Identity enables resilient documentation.

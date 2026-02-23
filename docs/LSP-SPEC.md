# LSP Specification

---

## 1. Purpose

This document defines how the Language Server Protocol (LSP) integrates into the system.

LSP is an optional semantic enhancement layer.

Tree-sitter provides structure.

LSP provides semantics.

LSP enables:

* Accurate definition resolution
* Cross-file symbol resolution
* Type resolution
* Semantic reference lookup
* Language-aware symbol information

LSP is not mandatory for core operation, but when available, it improves accuracy significantly.

---

## 2. Role in Architecture

LSP operates alongside Tree-sitter in the Parsing Layer.

```text id="m3q3br"
Source File
   ↓
Tree-sitter → Structural Symbols
   ↓
LSP → Semantic Resolution
   ↓
Normalized Symbol Graph
```

Tree-sitter extracts structure.

LSP resolves meaning.

The Indexing Layer consumes merged output.

---

## 3. Responsibilities

LSP integration must support:

* go-to-definition
* find-references
* document symbols
* workspace symbols
* hover metadata (optional)
* type information (when available)

LSP must not:

* Replace UID generation logic
* Store documentation
* Own persistence layer

It is a semantic resolver only.

---

## 4. Supported LSP Capabilities

The system may leverage:

### 4.1 textDocument/documentSymbol

Used to extract structured symbol list per file.

Provides:

* Symbol name
* Kind
* Range
* Selection range
* Hierarchical structure

---

### 4.2 textDocument/definition

Used to resolve:

* Reference → definition mapping
* Cross-file resolution
* Accurate target symbol

---

### 4.3 textDocument/references

Used to:

* Retrieve all known references
* Validate Tree-sitter reference extraction
* Improve accuracy for dynamic languages

---

### 4.4 workspace/symbol

Used for:

* Global symbol search
* Workspace-wide resolution
* Fallback indexing support

---

### 4.5 textDocument/hover (Optional)

May provide:

* Type info
* Documentation string
* Signature details

Hover content is not treated as canonical documentation.

---

## 5. Integration Strategy

LSP must operate in one of two modes:

### Mode A: Hybrid Mode (Preferred)

* Tree-sitter extracts structure.
* LSP resolves semantic targets.
* System merges results.

This provides:

* Fast incremental parsing
* Accurate cross-file resolution

---

### Mode B: LSP-Only Mode

When Tree-sitter adapter is unavailable:

* Rely entirely on LSP symbol extraction.
* Limited structural fingerprinting.
* Reduced refactor resilience.

This is fallback mode.

---

## 6. UID Generation with LSP

UID generation must not depend on LSP session IDs.

UID inputs may include:

* Fully qualified name
* Module path
* Symbol kind
* Structural fingerprint from Tree-sitter

LSP assists resolution but does not define identity.

---

## 7. Cross-File Resolution

When LSP is available:

* Use definition response to map reference → canonical symbol.
* Ensure single UID per resolved symbol.
* Deduplicate references via UID matching.

If LSP is unavailable:

* Fallback to heuristic matching:

  * Name-based resolution
  * Import mapping
  * Scope-based inference

System must track resolution confidence.

---

## 8. Incremental Updates

When file changes:

1. Notify LSP of textDocument/didChange.
2. Re-run documentSymbol.
3. Update local Tree-sitter AST.
4. Compare LSP resolution changes.
5. Update symbol graph accordingly.

Index must reconcile:

* Structural change
* Semantic change

---

## 9. Workspace Indexing

For large projects:

* LSP may provide full workspace index.
* Alternatively import LSIF/SCIP.

LSP is not guaranteed to scale to massive monorepos.

System must handle:

* Partial LSP availability
* Large workspace limitations

---

## 10. Language Server Isolation

Each language:

* Runs its own LSP instance.
* Is isolated from other languages.
* Communicates via standardized protocol.

Core system must:

* Abstract LSP implementation details.
* Avoid language-specific logic leakage.

---

## 11. Performance Constraints

LSP calls must:

* Be cached when possible.
* Avoid redundant definition queries.
* Batch reference lookups when supported.
* Not block UI thread in IDE context.

System must implement:

* Timeout handling
* Graceful degradation
* Retry logic

---

## 12. Error Handling

LSP may:

* Crash
* Hang
* Return partial results
* Fail on incomplete code

System must:

* Detect LSP unavailability
* Switch to degraded mode
* Avoid index corruption
* Mark semantic confidence levels

---

## 13. Security Considerations

LSP servers:

* May execute language tooling.
* May trigger background processes.
* May depend on build systems.

System must:

* Sandbox LSP execution when possible.
* Avoid executing untrusted scripts.
* Prevent code execution via index layer.

---

## 14. Limitations of LSP

LSP is:

* Language-specific
* Dependent on language server quality
* Not guaranteed deterministic across versions
* Not always incremental-efficient
* Not designed as persistent index

Therefore:

LSP enhances the system.

It does not replace the core index.

---

## 15. Relationship with LSIF / SCIP

For large repositories:

* Prefer precomputed index via LSIF or SCIP.
* Use LSP for live updates.
* Merge LSIF baseline with incremental LSP deltas.

This enables:

* CI-generated stable index
* Local incremental refinement

---

## 16. Testing Requirements

For LSP integration:

* Validate definition resolution accuracy.
* Validate reference completeness.
* Test rename scenarios.
* Test cross-module resolution.
* Test multi-language coexistence.

System must compare:

Tree-sitter reference extraction vs LSP resolution.

Discrepancies must be logged.

---

## 17. Future Extensions

Potential improvements:

* Symbol confidence scoring
* Multi-language cross-linking
* Type graph extraction
* Interface implementation mapping
* Override detection
* Dynamic dispatch approximation

These enhance semantic depth.

---

## 18. Summary

LSP provides:

* Semantic resolution
* Cross-file symbol mapping
* Type awareness
* Workspace intelligence

Tree-sitter provides:

* Structural parsing
* Incremental AST
* Deterministic fingerprinting

Together they form:

Structure + Semantics

The index layer unifies them.

LSP is optional but strongly recommended.

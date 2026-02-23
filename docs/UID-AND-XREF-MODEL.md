# UID and Cross-Reference Model

---

## 1. Purpose

This document defines:

* How UIDs are generated
* How identity stability is preserved
* How cross-references are resolved
* How links remain stable across refactors

UID is the foundation of the entire system.

Without stable identity, refactor-resistant documentation is impossible.

---

## 2. Identity Philosophy

Identity must be:

* Deterministic
* Stable under formatting changes
* Stable under reordering
* Stable under whitespace edits
* Stable under comment edits
* Resistant to minor refactors
* Regenerable from source alone

Identity must not depend on:

* Line numbers
* Byte offsets
* Indexing session
* Runtime memory addresses
* Editor state

UID must derive from semantic structure.

---

## 3. UID Structure

Canonical UID format:

```
sym:<language>:<module_path>:<qualified_name>:<struct_hash>
```

Example:

```
sym:ts:src/services/cart:CartService.calculateTotal:ab12cd34
```

Components:

* language → short code (ts, py, go, cs, rs)
* module_path → normalized project-relative path
* qualified_name → full namespace/class/method chain
* struct_hash → structural fingerprint

---

## 4. Structural Fingerprint

Structural hash must include:

* Node type
* Qualified name
* Parent chain
* Signature (if applicable)
* Parameter types (if available)
* Return type (if available)

Structural hash must exclude:

* Line numbers
* Whitespace
* Comments
* Formatting

Fingerprint example input:

```text id="cql2wo"
function_declaration
  name: calculateTotal
  parent: CartService
  parameters: [Item[]]
  return_type: number
```

Hash algorithm:

* Canonical JSON serialization
* Stable field order
* SHA-256 (or similar)

---

## 5. UID Stability Rules

### 5.1 Formatting Change

No UID change.

### 5.2 Comment Change

No UID change.

### 5.3 Function Reorder Within File

No UID change.

### 5.4 File Rename

Module path changes → UID changes.

Mitigation:

* Track file move via git diff.
* Provide UID remapping table.

### 5.5 Symbol Rename

Qualified name changes → UID changes.

System must:

* Detect rename via structural similarity.
* Provide remap event.

### 5.6 Signature Change

Signature change → struct_hash changes → UID changes.

Anchoring layer must attempt fuzzy reattachment.

---

## 6. UID Types

### 6.1 Symbol UID

```
sym:...
```

Primary identity of code constructs.

---

### 6.2 File UID

```
file:<normalized_path>
```

Stable until file move.

---

### 6.3 Module UID

```
module:<path_or_namespace>
```

Represents logical grouping.

---

### 6.4 Documentation UID

```
doc:<slug_or_uuid>
```

Stable, independent of code.

---

### 6.5 Concept UID

```
concept:<slug>
```

Abstract layer.

---

## 7. Cross-Reference Model

Cross-references connect:

* Symbol ↔ Symbol
* Symbol ↔ Documentation
* Documentation ↔ Concept
* Concept ↔ Symbol

All references must use UID.

No reference may rely solely on:

* Symbol name string
* File + line
* Text search

---

## 8. Reference Resolution Rules

Reference resolution process:

1. Extract identifier occurrence.
2. Resolve via LSP or LSIF.
3. Map to canonical Symbol UID.
4. Store reference edge.

If resolution fails:

* Mark as unresolved.
* Store resolution_confidence < 1.0.

---

## 9. Bidirectional Linking

For each reference:

* Forward link: caller → callee
* Reverse link: callee → callers

Reverse links must be indexed for:

* Impact analysis
* Documentation navigation
* Ranking

---

## 10. Documentation Cross-References

Documentation may include inline UID references.

Example:

```
See [[sym:ts:src/services/cart:CartService.calculateTotal:ab12cd34]] for calculation logic.
```

Renderer may convert to:

* Human-readable links
* Hover previews
* Graph navigation

UID remains canonical.

---

## 11. UID Migration Strategy

*(See [Anchoring Specification](ANCHORING-SPEC.md) for details on rebinding and confidence thresholds)*

When UID changes due to refactor:

System must:

1. Detect old UID no longer exists.
2. Search for structural similarity.
3. Attempt fuzzy match:

   * Same parent class
   * Similar signature
   * High subtree similarity
4. If match confidence ≥ threshold:

   * Rebind anchor
   * Emit remap event
5. If no match:

   * Mark anchor unresolved

No silent reassignment.

---

## 12. Refactor Scenarios

### Scenario A: Extract Method

Original method shrinks.

New method created.

New method gets new UID.

Original method retains UID if fingerprint unchanged.

---

### Scenario B: Move Method to Another Class

Qualified name changes.

UID changes.

Anchoring must attempt similarity-based remap.

---

### Scenario C: Rename Variable

Qualified name changes.

UID changes.

Low structural impact.

System may detect rename pattern and suggest remap.

---

### Scenario D: Parameter Added

Signature changes.

struct_hash changes.

UID changes.

Anchoring layer may match via partial similarity.

---

## 13. Cross-Repository Identity

When LSIF/SCIP provides global symbol IDs:

System must:

* Map external symbol → internal UID
* Preserve external ID as metadata
* Allow cross-repository linking

External symbols may use:

```
ext:<ecosystem>:<symbol_id>
```

---

## 14. Integrity Rules

The system must enforce:

* UID uniqueness
* Deterministic generation
* No duplicate UID for distinct symbols
* No reference to nonexistent UID
* No orphaned documentation without anchor check

---

## 15. XRef Map

System must maintain XRef map:

```json id="mtz6xo"
{
  "sym:CartService.calculateTotal": {
    "definitions": [...],
    "references": [...],
    "documentation": ["doc:cart-calc-overview"],
    "concepts": ["concept:pricing-engine"]
  }
}
```

XRef map must be:

* Efficiently queryable
* Cached
* Incrementally updated

---

## 16. Query Guarantees

Given UID:

System guarantees:

* Deterministic lookup
* Stable relationships
* Bounded reference list
* Confidence metadata

---

## 17. Security Considerations

UID must:

* Not expose system paths outside project
* Normalize paths
* Avoid injection via malformed identifier
* Validate allowed characters

---

## 18. Versioning

UID schema version must be stored.

If UID format changes:

* Require index rebuild
* Provide migration strategy

---

## 19. Non-Goals

UID system does not:

* Guarantee global uniqueness across all projects
* Prevent semantic duplication
* Replace version control
* Encode runtime identity

UID is structural identity, not runtime identity.

---

## 20. Summary

UID is:

* The backbone of identity.
* The anchor point for documentation.
* The key to refactor resilience.
* The foundation for cross-references.
* The guarantee of stable linking.

Without deterministic UID:

There is no reliable documentation graph.
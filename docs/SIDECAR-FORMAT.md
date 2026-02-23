# Sidecar Format Specification

---

## 1. Purpose

This document defines the sidecar documentation format.

The sidecar format governs:

* How documentation is stored outside source files
* How documentation binds to symbols
* How anchors are declared
* How documentation evolves
* How documentation is versioned

Sidecar documentation must be:

* Human-readable
* Machine-parseable
* Git-friendly
* Deterministic
* Explicitly bound to identity

---

## 2. Sidecar Principles

Sidecar documentation must:

* Not modify source files
* Not depend on inline comments
* Bind via UID or selector
* Be refactor-resilient
* Be version-controllable
* Be easy to diff

Sidecar must not:

* Store rendered HTML
* Embed AST snapshots
* Depend on line numbers
* Depend on transient offsets

---

## 3. Storage Location

Recommended repository layout:

```text
.project_root/
  src/
  docs-sidecar/
    symbols/
    concepts/
    architecture/
```

Alternative:

```text
.project_root/
  .sidecar-docs/
```

The storage directory must be configurable.

---

## 4. File Naming Convention

Each documentation unit stored as:

```text
doc-<slug>.md
```

Example:

```text
doc-cart-calc-overview.md
doc-pricing-engine.md
```

Slug must be:

* URL-safe
* Lowercase
* Deterministic or manually defined

---

## 5. Sidecar Document Structure

Each document contains:

1. YAML metadata header
2. Markdown body

Canonical format:

```markdown
---
doc_uid: doc:cart-calc-overview
title: Cart Calculation Logic
anchors:
  - anchor_type: symbol
    symbol_uid: sym:ts:src/services/cart:CartService.calculateTotal:ab12cd34
    fingerprint: abc123
    confidence: 1.0
related_concepts:
  - concept:pricing-engine
created_at: 2026-01-01T12:00:00Z
updated_at: 2026-01-02T15:00:00Z
version: 1
---

## Overview

This method calculates total cart cost including taxes...
```

---

## 6. Metadata Fields

### Required

* doc_uid
* title
* anchors

### Optional

* related_concepts
* tags
* authors
* status (draft, stable, deprecated)
* created_at
* updated_at
* version
* references (manual cross-doc links)

---

## 7. Anchor Declaration

Anchors declared explicitly in metadata.

### Symbol Anchor

```yaml
anchors:
  - anchor_type: symbol
    symbol_uid: sym:ts:src/services/cart:CartService.calculateTotal:ab12cd34
    fingerprint: abc123
    confidence: 1.0
```

---

### Structural Selector Anchor

```yaml
anchors:
  - anchor_type: structural_selector
    selector:
      node_type: function_declaration
      parent_chain:
        - type: class_declaration
          name: CartService
      name: calculateTotal
      signature: "(items: Item[]) => number"
      subtree_hash: abc123
    confidence: 0.92
```

---

### File-Level Anchor

```yaml
anchors:
  - anchor_type: file_level
    file_uid: file:src/services/cart.ts
```

---

## 8. Cross-References in Content

Content may reference UIDs inline:

```markdown
See [[sym:ts:src/services/cart:CartService.calculateTotal:ab12cd34]] for details.
```

Renderer may convert to:

* Link
* Hover preview
* Tooltip
* CLI reference

UID syntax must be:

```
[[<uid>]]
```

No implicit linking by name.

---

## 9. Concept Files

Concepts stored separately:

```text
concept-pricing-engine.md
```

Format:

```markdown
---
uid: concept:pricing-engine
title: Pricing Engine
related_symbols:
  - sym:ts:src/services/cart:CartService.calculateTotal:ab12cd34
---

Pricing engine handles tax rules and discount logic.
```

Concepts are optional but recommended.

---

## 10. Directory Organization

Recommended structure:

```text
docs-sidecar/
  symbols/
    cart/
      doc-cart-calc-overview.md
  concepts/
    concept-pricing-engine.md
  architecture/
    doc-payment-flow.md
```

Structure is organizational only.

UID is canonical identity.

---

## 11. Validation Rules

Sidecar files must:

* Contain valid YAML header
* Include doc_uid
* Include at least one anchor
* Not contain duplicate doc_uid
* Not reference nonexistent UID (warn only)
* Not exceed configured size limits

Validation must run during indexing.

---

## 12. Versioning

Each document may include:

* version field
* updated_at timestamp

Version increments optional.

Sidecar must not auto-overwrite documentation.

---

## 13. Merge Conflict Handling

Sidecar format must be:

* Line-based diff friendly
* Human-editable
* Deterministic key order

YAML fields must be canonicalized:

* Alphabetical order
* Stable formatting

This reduces merge conflicts.

---

## 14. Documentation Evolution

When anchor rebind occurs:

System must:

* Update symbol_uid in metadata
* Update fingerprint
* Update confidence
* Log migration event

Sidecar file must be modified deterministically.

---

## 15. CLI Compatibility

CLI must support:

* Create doc
* Attach to symbol
* Rebind anchor
* Validate anchors
* List unresolved anchors
* Update metadata
* Rename doc slug
* Move doc file

CLI must not auto-edit content body.

---

## 16. Security Considerations

Sidecar parser must:

* Sanitize YAML
* Reject arbitrary code execution
* Prevent path traversal
* Validate UID format
* Reject oversized documents

No embedded scripts allowed.

---

## 17. Migration Strategy

If schema changes:

* Update version field
* Provide migration CLI
* Maintain backward compatibility when possible
* Fail gracefully if incompatible

---

## 18. Extensibility

Future additions may include:

* Embedded diagrams
* Structured sections
* Machine-generated summary field
* Confidence scoring for documentation quality
* Coverage metadata

Extensibility must not break anchor invariants.

---

## 19. Non-Goals

Sidecar format does not:

* Replace markdown rendering systems
* Generate static websites by itself
* Store entire symbol graph
* Embed AST
* Replace version control

Sidecar is documentation store only.

---

## 20. Summary

Sidecar format provides:

* Human-readable documentation
* Explicit UID binding
* Deterministic anchoring
* Git-friendly structure
* Refactor resilience support

Source code evolves.

Sidecar documentation remains stable and attached.

This separation is foundational.
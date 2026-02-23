# Anchoring Specification

---

## 1. Purpose

This document defines how documentation is bound to code in a refactor-resilient manner. Anchoring is the critical mechanism that ensures documentation remains attached to the correct symbol, surviving formatting changes, reordering, and partial refactors. It allows documentation to be deterministically reattached or to intelligently detect when an attachment becomes invalid. Ultimately, robust anchoring is the primary defense against documentation rot.

---

## 2. Anchoring Principles

Anchoring must be:

* Symbol-first
* Structural-aware
* Diff-aware
* Confidence-scored
* Deterministic
* Auditable

Anchoring must not rely solely on:

* Line numbers
* Raw text offsets
* Plain-text search

---

## 3. Anchor Types

The system supports multiple anchor strategies.

### 3.1 Symbol Anchor (Primary)

Binds documentation to a Symbol UID.

```json id="m8b2qr"
{
  "anchor_type": "symbol",
  "symbol_uid": "sym:ts:src/services/cart:CartService.calculateTotal:ab12cd34",
  "fingerprint": "structural_hash",
  "confidence": 1.0
}
```

This is the preferred anchoring method.

---

### 3.2 Structural Selector Anchor

Used when:

* Symbol UID unavailable
* Symbol not yet indexed
* Attaching to anonymous blocks

Selector includes:

* Node type
* Parent hierarchy
* Child index
* Subtree hash

Example:

```json id="tzf3xp"
{
  "anchor_type": "structural_selector",
  "node_type": "function_declaration",
  "parent_chain": ["class:CartService"],
  "signature": "(items: Item[]) => number",
  "fingerprint": "subtree_hash",
  "confidence": 0.9
}
```

---

### 3.3 File-Level Anchor

Used for:

* Module-level documentation
* Architectural notes

```json id="d8z3wl"
{
  "anchor_type": "file_level",
  "file_uid": "file:src/services/cart.ts"
}
```

---

### 3.4 Fuzzy Anchor

Fallback mechanism.

Used when:

* Symbol UID changed
* Structural selector partially matches

Stores similarity metrics.

```json id="p7r4nb"
{
  "anchor_type": "fuzzy_match",
  "previous_uid": "sym:old",
  "candidate_uid": "sym:new",
  "similarity_score": 0.83,
  "confidence": 0.83
}
```

---

### 3.5 Manual Override Anchor

Used when:

* Automatic reattachment fails
* User explicitly binds documentation

Manual overrides must be flagged.

---

## 4. Anchor Lifecycle

### 4.1 Creation

When documentation is created:

1. Resolve symbol UID.
2. Store structural fingerprint.
3. Store anchor metadata.
4. Set confidence to 1.0.

---

### 4.2 Verification

On re-index:

1. Check if UID exists.
2. If exists:

   * Validate structural fingerprint.
   * If matches → confidence 1.0.
   * If differs → attempt rebind.
3. If UID missing:

   * Attempt similarity search.

---

### 4.3 Rebinding

If UID invalid:

1. Search for candidate symbols:

   * Same parent
   * Similar name
   * Similar signature
   * High subtree similarity
2. Compute similarity score.
3. If score ≥ threshold:

   * Rebind anchor.
   * Update UID.
   * Record migration event.
4. Else:

   * Mark anchor unresolved.

---

## 5. Structural Fingerprint Role

Structural fingerprint enables:

* Rename detection
* Minor signature change tolerance
* Function move detection
* Reorder resilience

Fingerprint must:

* Ignore whitespace
* Ignore comments
* Normalize type names
* Normalize generics

Fingerprint must change on meaningful semantic change.

---

## 6. Confidence Model

Each anchor must include confidence score:

* 1.0 → Exact match
* 0.8–0.99 → High-confidence reattachment
* 0.5–0.79 → Weak match (requires review)
* < 0.5 → Unresolved

System must:

* Expose confidence in query responses
* Allow filtering by confidence
* Log anchor changes

---

## 7. Refactor Scenarios

### 7.1 Reformat Code

No anchor change.

Confidence remains 1.0.

---

### 7.2 Reorder Methods

No anchor change.

Confidence remains 1.0.

---

### 7.3 Rename Method

UID changes.

Fingerprint mostly same.

Similarity high.

System rebinds.

Confidence ~0.95.

---

### 7.4 Extract Method

Original method shrinks.

Fingerprint partially changes.

System verifies:

* If still matches core signature.
* Else may reduce confidence.

New extracted method gets no documentation unless explicitly bound.

---

### 7.5 Move Method to Another Class

Qualified name changes.

Fingerprint same.

System detects similarity.

Rebind.

Confidence high.

---

### 7.6 Delete Symbol

No match found.

Anchor marked unresolved.

Confidence 0.0.

Documentation flagged for review.

---

## 8. Diff-Aware Anchoring

System may leverage:

* Git diff metadata
* AST diff
* Rename detection
* File move detection

Diff-aware flow:

1. Detect file move.
2. Update module path.
3. Preserve structural hash.
4. Attempt UID remap before fuzzy search.

Diff signals improve reattachment precision.

---

## 9. Anchoring Storage Format

Anchors stored as part of documentation metadata:

```json id="z2v1lu"
{
  "doc_uid": "doc:cart-calc-overview",
  "anchors": [
    {
      "anchor_type": "symbol",
      "symbol_uid": "...",
      "fingerprint": "...",
      "confidence": 1.0
    }
  ]
}
```

Multiple anchors allowed per documentation unit.

---

## 10. Query Behavior

When querying documentation:

System must:

* Verify anchor integrity
* Return anchor confidence
* Optionally include reattachment history

Queries must never assume anchor correctness blindly.

---

## 11. Anchor History

System must log:

* Old UID
* New UID
* Timestamp
* Similarity score
* Migration reason

Example:

```json id="x3k7nr"
{
  "event": "anchor_rebound",
  "old_uid": "sym:old",
  "new_uid": "sym:new",
  "similarity": 0.91,
  "timestamp": "..."
}
```

Audit trail is mandatory.

---

## 12. Integrity Constraints

The system must ensure:

* No documentation without anchor or explicit file-level binding
* No silent UID changes
* No untracked rebind
* No duplicate anchor records
* Deterministic fingerprint calculation

---

## 13. Failure Modes

Anchor may fail due to:

* Major refactor
* Symbol split
* Signature rewrite
* Type change
* File deletion

System must:

* Mark anchor unresolved
* Expose in diagnostics
* Not delete documentation automatically

Documentation must survive code deletion.

---

## 14. UI Implications

IDE and CLI must:

* Highlight low-confidence anchors
* Provide rebind suggestions
* Allow manual reassignment
* Display anchor health metrics

---

## 15. Non-Goals

Anchoring does not:

* Guarantee semantic equivalence
* Detect business logic changes
* Understand runtime behavior
* Replace code review

Anchoring tracks structural identity.

---

## 16. Security Considerations

Anchoring must:

* Validate UID format
* Avoid injection via fingerprint fields
* Normalize file paths
* Prevent external UID spoofing

---

## 17. Extensibility

Future enhancements:

* ML-assisted similarity scoring
* Semantic diff integration
* Structural tree edit distance metrics
* Cross-language anchor linking

These improve precision but do not redefine model.

---

## 18. Summary

Anchoring is:

* The durability layer of documentation.
* The guardian against rot.
* The mechanism for surviving refactors.
* The bridge between static identity and evolving code.

Without anchoring:

Documentation becomes stale.

With anchoring:

Documentation becomes resilient.
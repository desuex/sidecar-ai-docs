# AST Diff and Rebase Specification

---

## 1. Purpose

This document defines how the system:

* Detects structural changes in code
* Computes AST-level diffs
* Preserves symbol identity where possible
* Rebases anchors and UIDs after refactors
* Avoids unnecessary UID invalidation

AST diffing is critical for:

* Refactor resilience
* Stable anchoring
* Deterministic identity migration
* Minimal documentation breakage

Line-based diff is insufficient.

Structure must be compared structurally.

---

## 2. Why AST Diff

Traditional diff:

* Compares text lines
* Sensitive to formatting
* Sensitive to reorder
* Cannot detect semantic moves

AST diff:

* Compares structure
* Ignores formatting
* Detects node moves
* Detects renames
* Detects signature changes
* Detects subtree equivalence

AST diff enables:

Structural change detection independent of text noise.

---

## 3. Diff Inputs

AST diff requires:

* Previous AST snapshot
* New AST snapshot
* Node fingerprints
* Symbol UID mapping

Snapshots must include:

* Node type
* Qualified name
* Structural fingerprint
* Parent chain
* Signature (if available)

---

## 4. Node Identity During Diff

Each AST node must be assigned:

* Temporary structural ID
* Structural fingerprint
* Parent UID
* Kind

Diff algorithm attempts to:

* Match old nodes to new nodes
* Compute similarity score
* Detect move vs rename vs rewrite

---

## 5. Diff Categories

Structural changes fall into:

### 5.1 Unchanged

* Same fingerprint
* Same parent chain
* Same signature

UID remains stable.

---

### 5.2 Moved

* Same fingerprint
* Different parent chain
* Similar context

Example:
Method moved to another class.

UID likely changes due to qualified name change.

System may:

* Preserve structural identity
* Trigger UID remap
* Rebind anchors automatically

---

### 5.3 Renamed

* Same structure
* Different name
* Similar parameters
* Similar body

Detected via:

* Subtree similarity
* Body similarity threshold
* Parent consistency

---

### 5.4 Modified

* Fingerprint changes
* Subtree similar above threshold

Minor changes:

* Parameter rename
* Formatting
* Small body change

Major changes:

* Signature change
* Return type change
* Logic rewrite

System must compute change severity.

---

### 5.5 Split

* One old node maps to multiple new nodes

Example:
Extract method refactor.

System must:

* Keep original UID if core identity preserved
* Assign new UID to extracted node
* Avoid incorrect rebinding

---

### 5.6 Deleted

* Old node not found in new AST

Anchors become unresolved unless fuzzy match found.

---

## 6. Similarity Scoring

Similarity score computed from:

* Node type match
* Parameter count match
* Signature similarity
* Subtree hash similarity
* Body AST edit distance
* Parent similarity
* Child structure similarity

Example scoring weights:

```text
Node type match: +0.2
Name similarity: +0.2
Signature similarity: +0.3
Subtree similarity: +0.3
```

Score range: 0.0 – 1.0

Thresholds:

* ≥ 0.9 → strong match
* ≥ 0.75 → probable match
* < 0.75 → weak match

Threshold configurable.

---

## 7. Rebase Workflow

When re-indexing:

1. Parse new AST.
2. Load previous AST snapshot.
3. Compute structural diff.
4. Categorize changes.
5. Attempt UID preservation.
6. Rebind anchors.
7. Emit migration events.
8. Update index.

No silent UID reassignment.

---

## 8. UID Rebase Strategy

If symbol qualifies as:

### Exact structural match

Keep UID unchanged.

---

### Rename with high similarity

Generate new UID.
Create UID remap entry:

```json
{
  "old_uid": "...",
  "new_uid": "...",
  "reason": "rename",
  "similarity": 0.94
}
```

Update documentation anchors.

---

### Move with same structure

Treat similarly to rename.

---

### Minor modification

Keep UID stable if:

* Signature unchanged
* Subtree similarity high

---

### Major rewrite

New UID generated.
Old UID archived.
Anchors marked unresolved or fuzzy.

---

## 9. AST Snapshot Storage

System must store:

* Canonical structural snapshot
* Minimal tree representation
* Symbol-level subtree hashes
* UID mapping state

Snapshots may be:

* Versioned
* Stored per file
* Garbage collected when stale

---

## 10. Performance Constraints

AST diff must:

* Operate per-file
* Avoid whole-repo diff
* Complete under 100ms for typical file
* Avoid quadratic tree comparisons

Use:

* Hash-based subtree matching
* Node signature indexing
* Parent-child indexing

---

## 11. Git Integration

System may leverage:

* Git rename detection
* File move detection
* Commit metadata
* Diff hunks

Git diff improves:

* Confidence scoring
* Rename detection
* File move detection

But AST diff remains authoritative.

---

## 12. Conflict Handling

Ambiguous matches:

* Multiple candidates above threshold
* Similar sibling methods

System must:

* Select highest similarity
* Record ambiguity score
* Allow manual review

Never silently pick low-confidence match.

---

## 13. Anchor Rebase Rules

For each documentation anchor:

1. Check if UID still valid.
2. If not:

   * Run similarity search.
3. If match found above threshold:

   * Rebind.
   * Log migration.
4. Else:

   * Mark unresolved.

Anchor rebase must not delete documentation.

---

## 14. Logging and Audit

System must record:

* UID migrations
* Anchor rebindings
* Similarity scores
* Structural change classification
* Timestamps

Logs must be queryable.

Audit is mandatory.

---

## 15. Edge Cases

### Anonymous Functions

May lack stable qualified name.

Structural fingerprint becomes primary identity.

---

### Overloaded Methods

Signature differentiation required.

---

### Generated Code

May cause churn.

System must allow:

* Exclusion rules
* Ignore patterns
* Generated-file flag

---

### Macros / Preprocessing

May distort AST.

Fingerprint must be computed post-preprocessing when possible.

---

## 16. Failure Modes

AST diff may fail due to:

* Massive rewrite
* Code collapse
* Entire file replacement
* Parser error

In such cases:

* Mark all anchors unresolved
* Emit warning
* Avoid incorrect reattachment

---

## 17. Non-Goals

AST diff does not:

* Detect semantic equivalence
* Understand business logic
* Replace tests
* Replace code review

It tracks structural identity only.

---

## 18. Extensibility

Future enhancements:

* Tree edit distance algorithms
* ML-assisted similarity scoring
* Semantic diff (type-aware)
* Cross-language move detection
* Function body hashing optimizations

---

## 19. Summary

AST diff is:

* The engine of identity preservation.
* The foundation of refactor resilience.
* The bridge between old and new structure.
* The guardian against documentation breakage.

Without AST diff:

Refactors destroy documentation.

With AST diff:

Documentation survives evolution.

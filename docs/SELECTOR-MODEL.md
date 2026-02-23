# Selector Model Specification

---

## 1. Purpose

This document defines the structural selector model used to:

* Attach documentation to non-symbol structures
* Provide fallback anchoring
* Enable fine-grained structural targeting
* Support anonymous constructs
* Allow structural queries independent of UID

Selectors operate at the AST structure level.

They are secondary to Symbol UID anchors but critical for resilience.

---

## 2. Why Selectors Exist

Symbol UID anchoring works when:

* The construct has a stable name
* The symbol is indexable

Selectors are required when:

* Target is anonymous
* Target is partial structure
* Target is block-level construct
* Symbol identity unstable
* Documentation describes part of a function
* Documentation attaches to code pattern

Selectors enable structural binding beyond named symbols.

---

## 3. Selector Principles

Selectors must be:

* Deterministic
* Structural
* Serializable
* Language-aware
* Resilient to formatting
* Independent of line numbers
* Composable
* Verifiable

Selectors must not rely on:

* Raw text search
* Line offsets
* Whitespace structure

---

## 4. Selector Structure

A selector describes a path in the AST.

Canonical form:

```json id="t2f9qm"
{
  "node_type": "function_declaration",
  "qualified_parent_chain": [
    { "type": "class_declaration", "name": "CartService" }
  ],
  "name": "calculateTotal",
  "signature": "(items: Item[]) => number",
  "child_index": 2,
  "subtree_hash": "abc123"
}
```

All fields optional depending on context.

---

## 5. Selector Components

### 5.1 Node Type

Defines AST node category:

* function_declaration
* method_definition
* class_declaration
* block
* if_statement
* call_expression
* variable_declaration
* etc.

Node type must map to normalized AST node.

---

### 5.2 Parent Chain

Describes hierarchical containment.

Example:

```json id="e8qx7v"
[
  { "type": "module", "name": "services" },
  { "type": "class_declaration", "name": "CartService" }
]
```

Parent chain ensures uniqueness within file.

---

### 5.3 Name

Optional for named constructs.

Used for:

* Matching renamed symbols
* Increasing similarity score
* Improving resolution precision

---

### 5.4 Signature

Optional.

Includes:

* Parameter count
* Parameter types (if available)
* Return type

Helps differentiate overloads.

---

### 5.5 Child Index

Used when:

* Multiple anonymous blocks exist
* Name not sufficient

Represents position among siblings.

Must not rely on line number.

---

### 5.6 Subtree Hash

Structural fingerprint of subtree.

Computed from:

* Node type
* Children structure
* Normalized identifiers
* Excluding whitespace and comments

Used for:

* Exact match verification
* Similarity scoring
* Reattachment

---

## 6. Selector Types

### 6.1 Exact Selector

All components present.

Confidence = 1.0 if match found.

---

### 6.2 Partial Selector

Missing some fields.

Used when:

* Signature unavailable
* Parent chain incomplete

Confidence lower.

---

### 6.3 Pattern Selector

Used for attaching documentation to patterns.

Example:

```json id="v9hz3p"
{
  "node_type": "if_statement",
  "contains_call": "validateUser",
  "within_symbol": "sym:AuthService.login"
}
```

Pattern selectors are experimental.

---

### 6.4 Range-Constrained Selector

Optional range window.

Used as additional verification.

Must not be sole identity mechanism.

---

## 7. Selector Matching Algorithm

Given selector and new AST:

1. Filter nodes by node_type.
2. Filter by parent chain match.
3. Compare name (if present).
4. Compare signature (if present).
5. Compare subtree hash.
6. Compute similarity score.
7. Select highest confidence candidate.

---

## 8. Similarity Scoring

Similarity may include:

* Node type match (mandatory)
* Parent match weight
* Name similarity
* Signature similarity
* Subtree hash similarity
* Child structure similarity

Score range 0–1.

Threshold rules same as anchor rebasing.

---

## 9. Selector vs UID

UID:

* Represents named symbol identity.
* Preferred for documentation.

Selector:

* Represents structural target.
* Fallback or fine-grained anchor.

Selector may produce UID if target becomes named.

Selector may upgrade to UID anchor when possible.

---

## 10. Selector Persistence

Selectors stored inside documentation metadata:

```json id="x2k8qs"
{
  "doc_uid": "doc:cart-tax-note",
  "anchors": [
    {
      "anchor_type": "structural_selector",
      "selector": { ... },
      "confidence": 0.92
    }
  ]
}
```

Selectors must be serializable.

Selectors must not store entire AST.

---

## 11. Selector Use Cases

### 11.1 Documenting Anonymous Function

Example:

* Inline callback passed to map()

Selector uses:

* node_type: arrow_function
* parent context
* subtree hash

---

### 11.2 Documenting Critical If Block

Selector targets:

* if_statement
* within specific function
* subtree hash

---

### 11.3 Documenting Initialization Block

Selector targets:

* constructor
* child index
* specific pattern

---

## 12. Refactor Behavior

### Formatting change

Selector unaffected.

---

### Reorder siblings

If child_index used:

* May break.
* Use subtree hash to detect correct node.

---

### Rename parent class

Parent chain name changes.
Selector similarity may still detect via subtree.

---

### Extract block

Selector may become ambiguous.
Confidence drops.
Requires review.

---

## 13. Failure Handling

If selector cannot match:

* Mark anchor unresolved.
* Provide candidate list ranked by similarity.
* Allow manual reassignment.

No silent drop.

---

## 14. Performance Constraints

Selector matching must:

* Operate per-file.
* Avoid full tree traversal repeatedly.
* Use indexed node-type buckets.
* Cache subtree hashes.

Must not exceed acceptable indexing time.

---

## 15. Selector Validation

On each re-index:

* Validate selectors.
* Update confidence.
* Log changes.
* Report degraded anchors.

---

## 16. Security Considerations

Selectors must:

* Validate node types against whitelist.
* Prevent injection via malformed selector fields.
* Sanitize input.

Selectors must not allow arbitrary code execution.

---

## 17. Extensibility

Future extensions:

* Query DSL for selectors
* Pattern language (Tree-sitter query)
* Structural regex
* Multi-node selectors
* Cross-file structural selectors

These must remain deterministic.

---

## 18. Non-Goals

Selectors do not:

* Guarantee semantic equivalence.
* Replace UID for named constructs.
* Detect business meaning.
* Replace code review.

Selectors track structure only.

---

## 19. Summary

Selectors are:

* The structural fallback of anchoring.
* The tool for non-symbol binding.
* The mechanism for fine-grained documentation.
* The structural identity layer.

UID is identity of symbols.

Selectors are identity of structure.

Together they provide resilient documentation binding.

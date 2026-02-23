# Relevance Ranking Specification

---

## 1. Purpose

This document defines how the system ranks results for:

* Symbol search
* Documentation search
* Reference sampling
* Impact analysis prioritization
* Concept discovery
* Candidate rebind matches (anchor rebasing)

Ranking must be:

* Deterministic
* Transparent
* Stable
* Explainable
* Configurable (within bounds)
* Token-efficient

Ranking must not:

* Use randomness
* Depend on session state
* Depend on LLM interpretation
* Change unpredictably between runs

---

## 2. Design Principles

Ranking must:

1. Be score-based.
2. Be reproducible.
3. Be monotonic.
4. Avoid hidden heuristics.
5. Prefer signal over guesswork.
6. Avoid embedding black boxes by default.
7. Avoid dynamic learning unless explicitly enabled.

Ranking must optimize for:

* Reduced query iterations
* Minimal token expansion
* High-confidence first results

---

## 3. Determinism Guarantee

Given identical:

* Index state
* Query
* Parameters

Ranking must return identical ordered results.

Tie-breaking must be stable.

Tie-breaker order:

1. Higher score
2. Higher reference count
3. Lexicographically smaller UID
4. Stable insertion order

---

## 4. Symbol Search Ranking

### 4.1 Inputs

* Query string
* Optional symbol kind
* Optional language filter
* Optional module filter

---

### 4.2 Scoring Factors

Score components (example weights):

```text
Exact qualified_name match: +1.0
Exact name match: +0.9
Case-insensitive exact match: +0.85
Prefix match: +0.75
Substring match: +0.6
CamelCase match: +0.55
Fuzzy match (edit distance ≤ 2): +0.4
Kind match bonus: +0.1
Exported/public bonus: +0.05
Reference frequency (normalized): +0.05
```

Total score normalized to 0–1.

Weights must be configurable but deterministic.

---

### 4.3 Normalization

Reference frequency must be normalized across project:

```text
normalized_frequency = log(1 + reference_count) / log(1 + max_reference_count)
```

Prevents large projects from dominating.

---

## 5. Documentation Search Ranking

Documentation search may include:

* Title match
* Tag match
* Concept match
* Content substring match

Scoring factors:

```text
Title exact match: +1.0
Title prefix: +0.85
Tag exact match: +0.8
Concept exact match: +0.75
Content substring: +0.6
Content fuzzy match: +0.4
Anchor confidence bonus: +0.05
```

Results must prefer:

* High-confidence anchors
* Direct symbol binding
* Shorter match distance

---

## 6. Reference Sampling Ranking

When returning limited references:

Sampling must prioritize:

1. Direct callers in same module
2. Public entry points
3. Higher-level call sites
4. More frequently executed code (if available)
5. Deterministic lexical order fallback

Must not:

* Return random subset
* Bias toward recently indexed files

Sampling must be stable.

---

## 7. Impact Analysis Ranking

Impact results must prioritize:

1. Direct callers
2. High-reference symbols
3. Public/exported symbols
4. Entry points (main, controllers, handlers)
5. Cross-module references

Must include metadata:

```json
{
  "impact_level": "direct" | "transitive",
  "reference_count": 42
}
```

Depth must be explicitly controlled.

---

## 8. Anchor Rebinding Ranking

When rebinding anchors:

Scoring factors:

```text
Node type match (mandatory)
Signature similarity: weight 0.3
Subtree similarity: weight 0.3
Name similarity: weight 0.2
Parent similarity: weight 0.1
Child structure similarity: weight 0.1
```

Thresholds:

* ≥ 0.9 → strong match
* ≥ 0.75 → probable match
* < 0.75 → weak match

Highest scoring candidate selected.

Tie-breaking must be deterministic.

---

## 9. Concept Ranking

When listing concepts related to symbol:

Score factors:

```text
Direct binding: +1.0
Shared symbol count: +0.7
Documentation overlap: +0.5
Reference graph proximity: +0.3
```

Must not return arbitrary ordering.

---

## 10. Cross-Language Ranking

If multiple languages indexed:

Ranking must:

* Prefer current file language
* Prefer same module
* Avoid mixing unrelated ecosystems

Language context must be explicit.

---

## 11. Pagination Stability

Pagination must preserve ranking stability.

If:

* limit = 20
* offset = 0

And then:

* limit = 20
* offset = 20

Combined results must equal first 40 results of full ranking.

No re-ranking between pages.

---

## 12. No Hidden Re-ranking

System must not:

* Reorder based on access frequency
* Reorder based on recency
* Reorder based on LLM usage
* Use non-deterministic ML models

Ranking must remain static unless configuration changes.

---

## 13. Optional Embedding Layer (Future)

Embedding-based ranking may be added:

* As optional feature
* As secondary ranking layer
* Only when explicitly enabled

Embedding ranking must:

* Be deterministic given same embeddings
* Fall back to lexical ranking if disabled
* Never replace core lexical ranking silently

Embedding layer must not override UID determinism.

---

## 14. Transparency

CLI and MCP may expose score:

```json
{
  "uid": "...",
  "score": 0.87
}
```

For debugging and confidence.

Score visibility optional but recommended.

---

## 15. Performance Constraints

Ranking must:

* Operate within query time limits
* Avoid full-table scan when possible
* Use indexed fields
* Precompute reference counts
* Cache normalized frequency values

Must scale to >1M symbols.

---

## 16. Failure Modes

If ranking cannot compute score:

* Fall back to lexical exact match first
* Maintain deterministic ordering
* Emit warning in logs

Never silently degrade unpredictably.

---

## 17. Non-Goals

Ranking does not:

* Predict semantic importance
* Infer business meaning
* Replace developer intent
* Learn from AI usage
* Personalize results

It ranks relevance structurally and lexically only.

---

## 18. Summary

Relevance ranking must be:

* Deterministic
* Explainable
* Stable
* Token-efficient
* Structured
* Transparent

Ranking reduces:

* Query iterations
* Token expansion
* Cognitive load

Correct ranking is not cosmetic.

It is essential for token economy and developer trust.

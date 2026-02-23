# Token Economy Strategy

---

## 1. Purpose

This document defines how the system minimizes token usage and cognitive load when interacting with AI agents.

Token economy is not optional.

It is a first-class architectural goal.

The system must:

* Reduce context size
* Avoid redundant information
* Prevent graph explosion
* Support bounded retrieval
* Enable structured querying
* Prefer metadata over raw text
* Provide deterministic, minimal responses

The goal:

Minimal tokens.
Maximal structure.
Zero ambiguity.

---

## 2. Problem Statement

Raw codebases are large.

LLMs have:

* Context limits
* Cost constraints
* Performance degradation with size

Naive approaches:

* Dump full files
* Dump full symbol graphs
* Dump entire documentation corpus

Result:

* Token waste
* Hallucination risk
* Reduced reasoning clarity
* High cost

The system must instead:

Serve only what is necessary.

---

## 3. Token Efficiency Principles

The system must:

1. Prefer structured data over prose.
2. Return summaries before full content.
3. Require explicit expansion for large data.
4. Support field-level selection.
5. Support pagination.
6. Avoid recursive graph expansion.
7. Avoid implicit joins.
8. Use deterministic ranking.
9. Attach confidence metadata instead of verbose explanation.
10. Provide UID handles instead of large textual duplication.

---

## 4. Structured Over Text

Instead of returning:

```text
This method is used in 143 places across the project...
```

Return:

```json
{
  "reference_count": 143,
  "top_callers": [...]
}
```

AI can decide whether to request deeper expansion.

Structure is cheaper than prose.

---

## 5. Progressive Disclosure

All tools must follow:

Level 1: Metadata only
Level 2: Summary
Level 3: Full content

Example flow:

1. search_symbols → returns UID list.
2. get_symbol → returns metadata.
3. get_documentation → returns summary.
4. get_documentation(full=true) → returns full markdown.

Never skip levels automatically.

---

## 6. Field-Level Selection

All tools must support:

```json
{
  "fields": ["uid", "qualified_name"]
}
```

This prevents unnecessary fields like:

* Full signature
* Parent chain
* Reference lists
* Content body

Field selection is mandatory for large queries.

---

## 7. Pagination Discipline

All list queries must enforce:

* limit
* offset

Default limits must be conservative.

Example:

* find_references default limit = 20
* search_symbols default limit = 20

Token explosion prevention is mandatory.

---

## 8. Bounded Graph Traversal

Impact analysis must:

* Limit recursion depth
* Limit node count
* Require explicit depth parameter

Default:

```json
{
  "depth": 1
}
```

Never return entire transitive closure by default.

---

## 9. UID as Compression Primitive

UID replaces repeated text.

Instead of:

```text
CartService.calculateTotal in src/services/cart.ts
```

Return:

```json
{
  "uid": "sym:ts:src/services/cart:CartService.calculateTotal:ab12cd34"
}
```

UID is:

* Compact
* Deterministic
* Reusable
* Context-resolvable

AI may request expansion only when needed.

---

## 10. Summary-First Documentation

get_documentation must:

* Return summary by default
* Return full markdown only when requested

Summary may be:

* First paragraph
* Precomputed short description
* Bounded length (e.g., 300–500 chars)

Full content must require explicit flag.

---

## 11. Reference Compression Strategy

Instead of returning:

* 143 full reference objects

Return:

```json
{
  "total": 143,
  "sample": [...],
  "truncated": true
}
```

AI may request:

* Next page
* Specific reference UID
* Only file_uids

Sampling prevents overload.

---

## 12. Confidence Metadata Instead of Explanation

Instead of:

```text
The system believes this symbol may have been renamed...
```

Return:

```json
{
  "confidence": 0.87,
  "rebind_suggested": true
}
```

LLM can interpret.

Structured signals are cheaper than explanation.

---

## 13. No Implicit Expansion

MCP must never:

* Automatically include documentation when fetching symbol
* Automatically include references unless requested
* Automatically include parent tree
* Automatically include related concepts

All expansion must be explicit.

---

## 14. Caching Strategy for Token Reduction

At AI layer:

* Cache UID → short summary
* Cache symbol metadata
* Cache reference counts

Avoid repeated identical calls.

System must support deterministic caching.

---

## 15. Avoid Redundant Text Duplication

System must not:

* Duplicate signature in multiple responses
* Duplicate file path in nested structures
* Repeat parent chain unless requested

Repeated text inflates token usage.

---

## 16. Ranking for Minimal Exploration

Search tools must rank deterministically:

* Exact match first
* Public symbols prioritized
* Higher reference count ranked higher

Better ranking reduces need for multiple queries.

Fewer queries = fewer tokens.

---

## 17. Avoid Context Over-Expansion

System must prevent:

* Returning entire module graph
* Returning entire project symbol list
* Returning entire documentation corpus
* Returning entire file contents

Large expansions must require explicit user intent.

---

## 18. Token Budget Awareness

Tools may optionally accept:

```json
{
  "max_tokens": 800
}
```

Server may:

* Truncate content
* Reduce reference sample
* Omit low-priority fields

Always include:

```json
{
  "truncated": true
}
```

Transparency required.

---

## 19. Cognitive Economy

Token economy also reduces human cognitive load.

CLI and IDE must:

* Show summaries first
* Collapse large lists
* Highlight key metadata
* Avoid verbose explanations

The system must respect attention.

---

## 20. Deterministic Compression

Compression must be:

* Predictable
* Stable
* Transparent

No adaptive compression based on AI guesswork.

No hidden heuristics.

---

## 21. Anti-Patterns to Avoid

The system must avoid:

* Dumping entire AST
* Dumping entire file content
* Dumping entire reference graph
* Returning large markdown without truncation
* Returning duplicate fields
* Returning nested graph by default

These are token anti-patterns.

---

## 22. Metrics and Observability

System should track:

* Average response size (bytes)
* Average reference list size
* Truncation frequency
* Most expensive tool calls
* Token-equivalent size estimates

Optimization must be data-driven.

---

## 23. Extensibility

Future improvements:

* Embedding-based compressed summaries
* Symbol clustering
* Graph centrality precomputation
* Adaptive ranking
* Token-aware batching

Extensions must preserve determinism.

---

## 24. Non-Goals

Token economy does not:

* Remove necessary information
* Hide data
* Replace explicit user requests
* Compromise correctness
* Limit developer access artificially

It optimizes access, not restricts it.

---

## 25. Summary

Token economy strategy ensures:

* Minimal structured responses
* Progressive disclosure
* UID-based compression
* Deterministic bounded retrieval
* No implicit expansion
* Reduced hallucination risk
* Lower cost
* Faster reasoning

The system must never treat token cost as secondary.

Structure is cheaper than prose.

Explicit is cheaper than implicit.

Deterministic is cheaper than exploratory.

Token economy is architecture, not optimization.
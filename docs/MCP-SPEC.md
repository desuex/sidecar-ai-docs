# MCP Specification (Model Context Protocol Integration)

---

## 1. Purpose

This document defines how the system exposes its indexed knowledge to AI agents using MCP (Model Context Protocol).

MCP integration enables:

* Token-efficient context retrieval
* Structured symbol queries
* Deterministic data access
* Bounded responses
* Tool-based interaction
* Minimal prompt bloat

The system must be AI-native without being AI-dependent.

---

## 2. Design Goals

MCP layer must:

* Expose structured queries
* Return minimal required fields
* Avoid large text dumps
* Support pagination
* Support relevance scoring
* Support partial selection
* Support structured filtering
* Be stateless per request
* Be deterministic

MCP must not:

* Dump entire index
* Return raw file contents unnecessarily
* Depend on free-form LLM search

---

## 3. MCP Role in Architecture

```text
AI Agent
   ↓
MCP Tool Call
   ↓
Query Layer
   ↓
Index Storage
   ↓
Structured Response
```

MCP sits above storage and below AI.

It is an interface layer.

---

## 4. Core MCP Tools

Current MVP tools:

1. get_symbol
2. find_references
3. get_documentation
4. search_symbols
5. coverage_metrics
6. detect_undocumented_symbols

Planned additions:

1. impact_analysis
2. validate_anchors
3. list_unresolved_docs
4. get_concept
5. search_docs
6. get_file_summary

Each tool must:

* Accept structured parameters
* Return structured JSON
* Limit response size

---

## 5. Tool Specifications

---

### 5.1 get_symbol

Retrieve symbol metadata.

Input:

```json
{
  "uid": "sym:..."
}
```

Output:

```json
{
  "uid": "...",
  "name": "...",
  "qualified_name": "...",
  "kind": "...",
  "signature": "...",
  "file_uid": "...",
  "parent_uid": "...",
  "documentation_links": ["doc:..."]
}
```

---

### 5.2 find_references

Find references to symbol.

Input:

```json
{
  "uid": "sym:...",
  "limit": 50,
  "offset": 0
}
```

Output:

```json
{
  "total": 120,
  "references": [
    {
      "from_uid": "...",
      "file_uid": "...",
      "type": "call"
    }
  ]
}
```

Must support pagination.

---

### 5.3 get_documentation

Retrieve documentation.

Input:

```json
{
  "doc_uid": "doc:..."
}
```

Output:

```json
{
  "doc_uid": "...",
  "title": "...",
  "content": "...",
  "anchors": [...],
  "confidence": 1.0
}
```

---

### 5.4 search_symbols

Search by name or filter.

Input:

```json
{
  "query": "calculate",
  "kind": "method",
  "limit": 20
}
```

Output:

```json
{
  "results": [
    {
      "uid": "...",
      "qualified_name": "...",
      "relevance_score": 0.87
    }
  ]
}
```

Relevance must be deterministic.

---

### 5.5 coverage_metrics

Compute documentation coverage from indexed symbols.

Input:

```json
{
  "public_only": true,
  "scan_limit": 5000
}
```

Output:

```json
{
  "public_only": true,
  "scan_limit": 5000,
  "scan_complete": true,
  "scanned_symbols": 124,
  "eligible_symbols": 97,
  "documented_symbols": 71,
  "undocumented_symbols": 26,
  "coverage_pct": 73.2
}
```

---

### 5.6 detect_undocumented_symbols

Return undocumented symbols in deterministic order.

Input:

```json
{
  "public_only": true,
  "scan_limit": 5000,
  "limit": 50,
  "offset": 0
}
```

Output:

```json
{
  "undocumented_total": 26,
  "results": [
    {
      "uid": "...",
      "name": "...",
      "kind": "function"
    }
  ],
  "truncated": true
}
```

---

### 5.7 impact_analysis

Find affected symbols.

Input:

```json
{
  "uid": "sym:..."
}
```

Output:

```json
{
  "direct_callers": [...],
  "transitive_callers": [...],
  "documentation_units": [...]
}
```

Must not recursively dump entire graph by default.

Depth parameter optional.

---

### 5.8 validate_anchors

Input:

```json
{
  "doc_uid": "doc:..."
}
```

Output:

```json
{
  "valid": true,
  "confidence": 0.97,
  "warnings": []
}
```

---

### 5.9 list_unresolved_docs

Output:

```json
{
  "unresolved": [
    {
      "doc_uid": "...",
      "reason": "symbol_not_found"
    }
  ]
}
```

---

### 5.10 get_concept

Retrieve concept.

---

### 5.11 search_docs

Search documentation by content.

Must support:

* Full-text search
* Tag filtering
* Concept filtering

---

### 5.12 get_file_summary

Return summary metadata only.

Must not dump entire file.

---

## 6. Token Efficiency Strategy

MCP responses must:

* Omit large content unless requested
* Support field selection
* Support summary-only mode
* Support depth control
* Support relevance ranking
* Avoid redundant nesting

Agent must request additional detail explicitly.

---

## 7. Partial Field Selection

All tools should support:

```json
{
  "fields": ["uid", "qualified_name"]
}
```

Prevents unnecessary data transfer.

---

## 8. Pagination

All list-returning tools must support:

* limit
* offset

Default limit must be conservative.

---

## 9. Ranking and Relevance

Relevance scoring may use:

* Exact name match
* Qualified name match
* Reference frequency
* Documentation coverage
* Graph centrality

Ranking must be deterministic.

---

## 10. Determinism

MCP responses must:

* Be deterministic for same input
* Not depend on runtime randomness
* Not reorder results arbitrarily
* Not depend on LLM interpretation

MCP is structured API, not free-form output.

---

## 11. CLI Parity

All MCP tools must have CLI equivalents.

CLI must:

* Accept JSON input
* Output JSON
* Support piping
* Support scripting
* Support batch mode

MCP and CLI must share same query core.

---

## 12. Security Model

MCP must:

* Validate UID inputs
* Sanitize filters
* Prevent injection
* Restrict file path traversal
* Enforce query limits
* Avoid arbitrary code execution

No tool may execute source code.

---

## 13. Rate Limits and Safeguards

MCP must:

* Cap maximum references returned
* Cap recursion depth
* Cap documentation size
* Reject overly broad queries

Protection against token explosion is mandatory.

---

## 14. Observability

MCP layer should log:

* Tool calls
* Query duration
* Result size
* Errors
* Anchor warnings

Metrics help optimize token efficiency.

---

## 15. Extensibility

Future MCP tools may include:

* suggest_documentation
* generate_summary
* graph_export

LLM-powered tools must not override deterministic tools.

---

## 16. Non-Goals

MCP does not:

* Replace storage layer
* Replace index layer
* Execute code
* Perform arbitrary LLM reasoning
* Generate documentation implicitly

It only exposes structured data.

---

## 17. Summary

MCP integration provides:

* Structured AI access
* Token-efficient queries
* Deterministic responses
* Controlled context expansion
* CLI parity
* Refactor-safe documentation retrieval

It transforms the index into an AI-native knowledge graph.

Without MCP:

AI must scrape text.

With MCP:

AI queries structure directly.

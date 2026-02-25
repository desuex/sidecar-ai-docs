# MCP Server Design

---

## 1. Purpose

This document defines the architecture of the MCP server implementation.

The MCP server:

* Exposes structured tools to AI agents
* Bridges index storage and AI interface
* Enforces token efficiency
* Ensures deterministic responses
* Applies query safeguards
* Maintains strict separation from business logic

The MCP server must remain thin and predictable.

---

## 2. High-Level Architecture

```text
AI Agent
   ↓
MCP Transport Layer
   ↓
Tool Dispatcher
   ↓
Query Engine
   ↓
Index Storage
   ↓
Structured Response
```

Each layer must be isolated and testable.

---

## 3. Core Components

The MCP server consists of:

1. Transport Layer
2. Tool Registry
3. Request Validator
4. Query Engine
5. Response Shaper
6. Safeguard Layer
7. Observability Module

---

## 4. Transport Layer

Responsible for:

* JSON-RPC or MCP protocol communication
* Tool registration
* Request parsing
* Response serialization

Must support:

* STDIO (default)
* HTTP (optional)
* WebSocket (optional)

Transport must be stateless.

No session memory allowed.

---

## 5. Tool Registry

The Tool Registry:

* Registers available tools
* Maps tool name → handler
* Defines input schema
* Defines output schema
* Validates parameters

Example registry entry:

```json
{
  "name": "get_symbol",
  "input_schema": {...},
  "output_schema": {...}
}
```

Schema validation must occur before execution.

---

## 6. Request Validator

The Request Validator must:

* Validate UID format
* Validate pagination bounds
* Validate field selection
* Enforce maximum limits
* Sanitize filter strings
* Reject malformed JSON
* Reject unknown tool calls

No tool executes before validation passes.

---

## 7. Query Engine

The Query Engine:

* Translates tool call → storage query
* Applies ranking
* Applies filters
* Applies pagination
* Collects metadata
* Handles confidence scoring

The Query Engine must:

* Be deterministic
* Avoid hidden heuristics
* Avoid implicit data expansion
* Avoid recursive explosion

---

## 8. Response Shaper

Response Shaper:

* Applies field selection
* Removes unused fields
* Enforces response size limits
* Attaches confidence metadata
* Adds pagination metadata
* Formats JSON deterministically

Must not reorder fields arbitrarily.

Must preserve schema.

---

## 9. Safeguard Layer

Safeguards must include:

* Max results per request
* Max recursion depth
* Max content size
* Max documentation length
* Timeout per query
* Query complexity cap

If safeguards triggered:

* Return truncated result
* Include truncation flag
* Provide guidance for pagination

---

## 10. Observability Module

Must log:

* Tool name
* Execution time
* Rows scanned
* Rows returned
* Response size (bytes)
* Error state
* Anchor confidence warnings

Logs must not include full documentation body unless debug mode enabled.

Metrics enable optimization.

---

## 11. Statelessness Requirement

MCP server must:

* Not maintain conversational state
* Not cache session memory
* Not store partial LLM context
* Not infer next queries

Each request must be independent.

---

## 12. Concurrency Model

Server must support:

* Concurrent read queries
* Single-writer indexing lock
* Non-blocking read during indexing when possible
* Graceful request queuing

Long-running queries must be cancellable.

---

## 13. Error Handling

Errors must be structured:

```json
{
  "error": {
    "code": "INVALID_UID",
    "message": "UID format invalid"
  }
}
```

Error types:

* INVALID_INPUT
* LIMIT_EXCEEDED
* NOT_FOUND
* INDEX_UNAVAILABLE
* TIMEOUT
* INTERNAL_ERROR

Errors must not expose internal file paths.

---

## 14. Determinism Guarantee

The MCP server must guarantee:

* Same input → same output
* Stable ordering
* Stable ranking
* Stable pagination
* No randomness

Determinism is critical for reproducible AI reasoning.

---

## 15. Field Selection Implementation

All list-returning tools must support:

```json
{
  "fields": ["uid", "qualified_name"]
}
```

Response Shaper enforces this by:

* Removing unspecified fields
* Preserving required fields

This reduces token cost.

---

## 16. Streaming Support (Optional)

Server may support streaming for:

* Large reference lists
* Graph traversal
* Search results

Streaming must:

* Preserve deterministic ordering
* Include total count header
* Allow cancellation

Streaming must not exceed safeguard limits.

---

## 17. Caching Strategy

Allowed caching:

* Symbol lookup cache
* Reference list cache
* Documentation metadata cache

Cache must:

* Be invalidated on index update
* Be version-aware
* Not cache partial query results incorrectly

No speculative caching allowed.

---

## 18. Index Synchronization

MCP server must detect:

* Index version mismatch
* Partial indexing state
* Ongoing reindex operation

If index unavailable:

* Return structured error
* Avoid partial inconsistent response

---

## 19. Security Considerations

MCP server must:

* Reject path traversal attempts
* Reject malformed UID injection
* Limit query complexity
* Enforce max payload size
* Prevent DoS via wide queries
* Sanitize YAML if sidecar loaded

No code execution allowed.

---

## 20. Deployment Modes

### 20.1 Local Development Mode

* Runs inside repository
* Accesses local index
* STDIO transport

---

### 20.2 CI Mode

* Exposes read-only index
* Possibly HTTP-based
* LSIF baseline import

---

### 20.3 IDE Mode

* Embedded MCP instance
* Reduced feature set
* Optimized for latency

---

## 21. Test Requirements

MCP server must include:

* Tool schema validation tests
* Determinism tests
* Pagination tests
* Field filtering tests
* Safeguard enforcement tests
* Error handling tests
* Anchor confidence reporting tests

---

## 22. Extensibility

Future additions:

* Semantic ranking enhancements
* Embedding-based search (optional)
* AI-assisted summarization tools
* Documentation coverage trend analysis
* Graph traversal API

Extensions must preserve determinism for core tools.

---

## 23. Non-Goals

MCP server does not:

* Execute code
* Run tests
* Compile source
* Generate documentation automatically
* Store AI conversation history

It is a structured index interface.

---

## 24. Summary

The MCP server is:

* The deterministic interface between AI and code graph.
* The guardian of token efficiency.
* The enforcer of structured queries.
* The protector against unbounded context explosion.
* The bridge between storage and reasoning.

It must remain:

Thin.
Predictable.
Safe.
Deterministic.

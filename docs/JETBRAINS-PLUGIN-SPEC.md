# JetBrains Plugin Specification

---

## 1. Purpose

This document defines the architecture and behavior of the JetBrains IDE plugin.

Supported IDEs:

* IntelliJ IDEA
* WebStorm
* PyCharm
* Rider
* GoLand
* CLion (where applicable)

The plugin must:

* Integrate with local MCP server
* Provide hover documentation
* Provide navigation to sidecar docs
* Show documentation coverage
* Show anchor health
* Respect JetBrains threading model
* Avoid blocking EDT (Event Dispatch Thread)

The plugin is a thin UI layer.

All logic lives in MCP + index core.

---

## 2. High-Level Architecture

```text
JetBrains IDE
  ↓
Plugin (Kotlin/Java)
  ↓
MCP Client (JSON-RPC over STDIO)
  ↓
Local MCP Server
  ↓
Index Storage
```

The plugin must not:

* Access index storage directly
* Reimplement AST parsing
* Reimplement indexing logic
* Replace PSI or LSP functionality

All queries must go through MCP tools.

---

## 3. Core Plugin Modules

The plugin must include:

1. Startup Activity
2. MCP Client Service
3. Hover Provider
4. Line Marker Provider
5. Code Insight Provider
6. Tool Window (Documentation Explorer)
7. Action Registry
8. Sidecar Validator
9. Status Indicator Service
10. Configuration Panel

Each module must be isolated.

---

## 4. Startup Behavior

On project open:

* Detect `.sidecar` or `docs-sidecar` directory
* Attempt to connect to local MCP server
* Show status in IDE status bar

If MCP unavailable:

* Show non-blocking notification
* Offer to start server

Startup must not block project loading.

---

## 5. MCP Client Service

Responsibilities:

* Spawn MCP server via ProcessBuilder
* Manage JSON-RPC communication
* Implement timeout handling
* Provide async request API
* Handle reconnection
* Validate response schema

Must:

* Run on background thread
* Use coroutines or CompletableFuture
* Support cancellation tokens

Timeout defaults:

* 1000ms for hover
* 3000ms for heavy queries

---

## 6. Hover Integration

Implemented via:

* DocumentationProvider or
* PsiElement documentation extension

Flow:

1. Detect PSI element under cursor.
2. Resolve symbol via PSI/LSP.
3. Request UID via `get_symbol`.
4. Request documentation via `get_documentation`.
5. Render HTML content.

Hover must:

* Show title
* Show short summary (first paragraph)
* Show anchor confidence
* Include link to open sidecar file

Content must be truncated (e.g., 500–800 chars).

---

## 7. Navigation to Documentation

Provide action:

* "Open Sidecar Documentation"

Behavior:

* Retrieve doc_uid via MCP
* Resolve sidecar file path
* Open in editor
* Navigate to metadata header

If multiple docs:

* Show popup selection list

---

## 8. Line Marker Provider

Used for:

* Documentation coverage icons
* Anchor health warnings

Icons:

* Green → documented (confidence ≥ threshold)
* Yellow → low confidence
* Red → unresolved anchor
* Gray → undocumented public symbol

Line markers must:

* Be lightweight
* Update on save
* Respect user settings
* Avoid clutter

---

## 9. Tool Window: Documentation Explorer

Optional but recommended.

Tool window may provide:

* Documentation tree
* Concept explorer
* Unresolved anchors list
* Coverage summary
* Search docs panel

Must use async loading.

Must not freeze UI.

---

## 10. Index Synchronization

On file save:

1. Debounce changes.
2. Trigger `sidecar update` (background).
3. Refresh markers.
4. Refresh anchor status.

Must not reindex entire project automatically.

---

## 11. PSI Integration

Plugin must:

* Use PSI tree to determine symbol boundaries
* Use built-in rename refactor events to detect potential UID changes
* Hook into refactor listeners when possible

Must not override native refactor behavior.

Must observe refactor events to refresh anchor diagnostics.

---

## 12. Anchor Health Feedback

If anchor confidence < threshold:

* Show warning icon in gutter
* Provide quick fix action:

  * "Rebind Anchor"
  * "Open Anchor Diagnostics"

Must not auto-rebind silently.

---

## 13. Performance Requirements

Plugin must:

* Avoid blocking EDT
* Keep hover latency < 100ms (cached)
* Cache UID lookups
* Cache doc existence
* Batch MCP calls when possible

Heavy operations require explicit user action.

---

## 14. Caching Strategy

Allowed caches:

* PSI element → UID
* UID → doc existence
* UID → short summary

Cache invalidation:

* On save
* On reindex
* On configuration change

Cache must not persist across sessions unless safe.

---

## 15. Configuration Panel

Provide settings under:

Preferences → Tools → Sidecar

Options:

* Enable hover integration
* Enable coverage markers
* Anchor confidence threshold
* MCP server path
* Auto-update on save
* Show anchor warnings

Defaults must be safe and non-intrusive.

---

## 16. Error Handling

If MCP server fails:

* Show balloon notification
* Provide "Reconnect" action
* Log detailed error in IDE log (not UI)

If index outdated:

* Show subtle status bar message
* Suggest running CLI update

Must not spam notifications.

---

## 17. Security Considerations

Plugin must:

* Only connect to local MCP server by default
* Validate JSON responses
* Not evaluate YAML dynamically
* Prevent path traversal when opening sidecar files
* Not execute arbitrary commands from MCP

No remote code execution allowed.

---

## 18. Testing Requirements

Plugin must include:

* Hover rendering tests
* Line marker tests
* MCP failure simulation tests
* Refactor scenario tests
* Anchor confidence update tests
* Performance baseline tests

Must test:

* Large projects
* Heavy reference graphs
* Rapid save events
* MCP restart scenarios

---

## 19. Non-Goals

JetBrains plugin does not:

* Replace LSP
* Replace PSI
* Reimplement indexing
* Generate documentation automatically
* Replace documentation website

It is a UI extension only.

---

## 20. Extensibility

Future features:

* Documentation graph visualization
* Concept relationship viewer
* Coverage heatmap overlay
* Inline documentation editor
* AI-assisted suggestions panel
* Refactor impact preview

Extensions must preserve performance and determinism.

---

## 21. Summary

The JetBrains plugin is:

* A thin client over MCP
* A contextual documentation renderer
* A refactor-aware anchor monitor
* A coverage visibility tool
* A non-intrusive IDE augmentation

All intelligence remains in:

Index + MCP core.

The plugin renders structure, not logic.

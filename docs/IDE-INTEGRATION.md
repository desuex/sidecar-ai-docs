# IDE Integration Specification

---

## 1. Purpose

This document defines how the system integrates with IDEs.

Supported IDE families:

* VS Code
* JetBrains IDEs (IntelliJ, WebStorm, PyCharm, Rider, etc.)

IDE integration must provide:

* Context-aware documentation
* Hover previews
* Symbol cross-reference navigation
* Anchor health visualization
* Documentation coverage indicators
* Minimal latency
* No token waste

IDE integration must remain a thin UI layer over the index + MCP core.

---

## 2. Design Principles

IDE integration must be:

* Non-intrusive
* Fast
* Deterministic
* Context-aware
* Refactor-safe
* Read-only by default
* Optional (does not affect core index)

IDE plugin must not:

* Modify source code automatically
* Inject comments
* Execute arbitrary code
* Replace language server
* Interfere with native LSP behavior

---

## 3. High-Level Architecture

```text
IDE
  ↓
Plugin
  ↓
MCP Client
  ↓
Local MCP Server
  ↓
Index Storage
```

Plugin is a UI + request layer.

All logic lives in core index + MCP server.

---

## 4. Core IDE Features

Minimum required features:

1. Hover Documentation
2. Jump to Documentation
3. Show References with Documentation Context
4. Documentation Coverage Indicators
5. Anchor Health Warnings
6. Inline UID Preview (optional)
7. Sidecar Editor Integration

---

## 5. Hover Documentation

When user hovers over a symbol:

1. Plugin resolves symbol via LSP.
2. Retrieves UID from index.
3. Calls `get_documentation`.
4. Displays summary.

Hover content must:

* Show documentation title
* Show first paragraph
* Show anchor confidence
* Provide link to full doc

Must not dump entire doc automatically.

Example hover:

```
CartService.calculateTotal
(method)

Calculates total cart cost including tax...

Documentation: doc:cart-calc-overview
Confidence: 1.0
```

---

## 6. Jump to Documentation

Command:

* "Open Sidecar Documentation"

Behavior:

1. Retrieve doc_uid.
2. Open corresponding sidecar file.
3. Scroll to beginning.
4. Highlight metadata header.

If multiple docs attached:

* Show picker list.

---

## 7. Reference Panel Enhancement

When user invokes "Find References":

Plugin may optionally:

* Fetch documentation summary for referenced symbol.
* Display doc link in reference panel.

Must not slow native LSP operation.

This feature must be optional.

---

## 8. Documentation Coverage Indicators

IDE may show:

* Gutter icons for documented symbols
* Highlight undocumented public symbols
* Display coverage percentage in status bar

Visual example:

* Green dot → documented
* Yellow dot → low confidence anchor
* Red dot → unresolved anchor
* Gray dot → undocumented

Must be lightweight.

---

## 9. Anchor Health Visualization

If anchor confidence < threshold:

* Show warning in gutter
* Provide quick action:

  * "Rebind Anchor"
  * "Open Anchor Diagnostics"

Must not auto-fix without confirmation.

---

## 10. Sidecar Editing Integration

When editing sidecar file:

* Validate YAML header
* Validate UID format
* Validate anchor structure
* Validate referenced symbol existence
* Highlight invalid anchors inline

Must not modify content body automatically.

---

## 11. Index Status Indicator

IDE status bar may show:

* Index ready
* Index outdated
* Index rebuilding
* MCP unavailable

If index outdated:

* Suggest running `sidecar update`

---

## 12. Refactor Handling

On rename or move:

* IDE LSP handles rename
* Index re-indexes
* Anchor rebasing occurs
* Plugin shows:

```
Documentation anchor updated:
sym:old → sym:new
Confidence: 0.94
```

Must allow review.

---

## 13. Performance Requirements

IDE integration must:

* Keep hover latency under 100ms
* Avoid blocking UI thread
* Use async requests
* Cache symbol lookups
* Avoid heavy graph traversal by default

Heavy queries must require explicit user action.

---

## 14. Offline Behavior

If MCP server unavailable:

* Hover fallback to minimal symbol info
* Show warning:
  "Sidecar index unavailable"

Must not crash.

---

## 15. VS Code Integration

VS Code plugin must:

* Use LanguageClient API
* Communicate via STDIO MCP server
* Use WebView only when necessary
* Avoid large memory footprint
* Follow VS Code extension guidelines

Commands must be registered via:

* package.json
* Command palette integration

---

## 16. JetBrains Integration

JetBrains plugin must:

* Use PSI model for symbol detection
* Integrate with existing LSP
* Use background thread for MCP queries
* Respect IDE threading model
* Use ToolWindow for documentation browser

Must avoid blocking EDT (Event Dispatch Thread).

---

## 17. Configuration Options

Plugin must allow:

* Enable/disable hover integration
* Enable/disable coverage markers
* Confidence threshold adjustment
* Auto-refresh on save toggle
* Index path override
* MCP endpoint override

Defaults must be safe.

---

## 18. Security Considerations

Plugin must:

* Only communicate with local MCP server by default
* Validate MCP responses
* Avoid executing returned content
* Avoid evaluating YAML dynamically
* Prevent path traversal in sidecar open

Must not expose project paths externally.

---

## 19. Non-Goals

IDE integration does not:

* Replace documentation site generator
* Replace version control
* Perform heavy semantic inference
* Auto-generate documentation
* Replace native LSP

It augments development workflow.

---

## 20. Extensibility

Future features:

* Inline documentation editing
* Graph view of symbol relationships
* Documentation coverage heatmap
* Concept explorer panel
* AI-assisted doc suggestions
* Refactor preview impact panel

Extensions must preserve performance and determinism.

---

## 21. Summary

IDE integration provides:

* Immediate contextual documentation
* Refactor-safe binding
* Visibility into documentation health
* Minimal friction workflow
* Deterministic structured access

The IDE becomes a live window into the code + documentation graph.

The core remains in index + MCP.

IDE is a thin client.
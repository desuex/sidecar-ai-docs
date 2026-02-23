# VS Code Extension Specification

---

## 1. Purpose

This document defines the architecture and behavior of the VS Code extension for the system.

The extension must:

* Integrate with local MCP server
* Provide hover documentation
* Provide documentation navigation
* Show coverage indicators
* Show anchor health
* Remain lightweight and non-blocking
* Respect VS Code extension API constraints

The extension must remain a thin UI layer.

All logic lives in MCP + index core.

---

## 2. Architectural Overview

```text
VS Code
  ↓
Extension (TypeScript)
  ↓
MCP Client (JSON-RPC / STDIO)
  ↓
Local MCP Server
  ↓
Index Storage
```

Extension must not:

* Access storage directly
* Implement custom parsing
* Reimplement indexing logic

All data access via MCP tools.

---

## 3. Core Extension Modules

The extension consists of:

1. Activation Module
2. MCP Client Module
3. Hover Provider
4. CodeLens Provider (optional)
5. Gutter Decoration Provider
6. Command Registry
7. Status Bar Module
8. Sidecar Validator
9. Configuration Manager

Each module must be isolated and testable.

---

## 4. Activation Strategy

Extension activation must occur on:

* Workspace open
* File open (supported language)
* Command invocation

Activation events:

```json
"activationEvents": [
  "onLanguage:typescript",
  "onLanguage:python",
  "onCommand:sidecar.openDocumentation",
  "workspaceContains:.sidecar"
]
```

Must avoid global activation for performance.

---

## 5. MCP Client Module

Responsibilities:

* Spawn MCP server via STDIO
* Handle JSON-RPC requests
* Implement request timeout
* Retry on failure
* Validate schema of responses
* Support cancellation tokens

Timeout defaults:

* 1000ms for hover
* 3000ms for heavy queries

Must not block event loop.

---

## 6. Hover Provider

When user hovers over a symbol:

1. Use VS Code's `vscode.executeDefinitionProvider`.
2. Map location to UID via MCP `get_symbol`.
3. Fetch documentation via `get_documentation`.
4. Render Markdown hover.

Hover must:

* Show title
* Show short summary (first paragraph)
* Show anchor confidence
* Provide clickable "Open Full Documentation"

Must limit content length (e.g., 500 chars).

---

## 7. CodeLens Provider (Optional)

Displays inline:

* "View Documentation"
* "Add Documentation"

Appears above symbol definitions.

CodeLens must:

* Be lightweight
* Cache symbol → doc existence
* Avoid excessive MCP calls

Must update on document save.

---

## 8. Gutter Decorations

Used for:

* Documentation coverage indicators
* Anchor health warnings

Icons:

* ✔ documented (confidence ≥ threshold)
* ⚠ low confidence anchor
* ✖ unresolved anchor
* ○ undocumented public symbol

Decorations must:

* Update on save
* Be toggleable in settings
* Avoid visual clutter

---

## 9. Commands

Extension must register commands:

```json
"contributes": {
  "commands": [
    { "command": "sidecar.openDocumentation", "title": "Open Sidecar Documentation" },
    { "command": "sidecar.createDocumentation", "title": "Create Documentation" },
    { "command": "sidecar.validateAnchors", "title": "Validate Anchors" }
  ]
}
```

Commands must:

* Call corresponding MCP tools
* Show structured errors
* Respect configuration

---

## 10. Status Bar Integration

Status bar may show:

* Index Ready
* Index Updating
* MCP Disconnected
* Unresolved Anchors (count)

Clicking status opens diagnostics panel.

Status must update reactively.

---

## 11. Sidecar File Integration

When editing sidecar `.md` files:

* Validate YAML header
* Validate UID format
* Validate anchor references
* Highlight invalid fields
* Provide quick fixes

Must not auto-correct without confirmation.

---

## 12. Index Sync Behavior

On file save:

1. Debounce events.
2. Trigger `sidecar update`.
3. Refresh coverage indicators.
4. Refresh anchor status.

Must avoid excessive reindexing.

---

## 13. Performance Requirements

Extension must:

* Keep hover latency < 100ms (cached)
* Avoid blocking extension host
* Cache symbol lookup results
* Batch MCP calls when possible
* Avoid full graph queries automatically

Heavy operations must require explicit command.

---

## 14. Caching Strategy

Allowed caches:

* Symbol → doc existence
* Symbol → anchor confidence
* Documentation summary (short)

Cache invalidation:

* On file save
* On index update
* On configuration change

Cache must not persist across sessions unless safe.

---

## 15. Configuration Settings

Extension must expose:

```json
"contributes": {
  "configuration": {
    "properties": {
      "sidecar.enableHover": { "type": "boolean", "default": true },
      "sidecar.enableCoverageIndicators": { "type": "boolean", "default": true },
      "sidecar.anchorConfidenceThreshold": { "type": "number", "default": 0.85 },
      "sidecar.mcpServerPath": { "type": "string" }
    }
  }
}
```

Defaults must be safe and non-intrusive.

---

## 16. Error Handling

If MCP server unavailable:

* Show notification:
  "Sidecar MCP server not running."
* Provide action: "Start MCP Server"

Errors must:

* Be concise
* Avoid technical stack traces
* Provide next-step guidance

---

## 17. Security Considerations

Extension must:

* Only connect to local MCP by default
* Validate response schema
* Not evaluate returned markdown as code
* Not execute arbitrary shell commands
* Prevent path traversal when opening sidecar files

No remote code execution.

---

## 18. Testing Requirements

Extension must include:

* Hover rendering tests
* Command invocation tests
* MCP failure tests
* Anchor indicator tests
* Configuration toggle tests
* Performance baseline tests

Must test on:

* Large repositories
* Slow MCP response
* Index rebuilding scenario

---

## 19. Non-Goals

The VS Code extension does not:

* Replace language server
* Execute indexing logic itself
* Generate documentation automatically
* Replace documentation website
* Persist documentation state

It is a UI client.

---

## 20. Extensibility

Future enhancements:

* Graph view panel
* Inline documentation editor
* Documentation coverage heatmap
* AI-assisted doc suggestions
* Impact analysis visualization
* Concept explorer sidebar

Extensions must preserve performance and determinism.

---

## 21. Summary

The VS Code extension is:

* A thin client over MCP
* A contextual documentation viewer
* A refactor-aware anchor monitor
* A documentation coverage indicator
* A safe, non-invasive augmentation of developer workflow

All intelligence resides in:

Index + MCP.

The extension only renders structure.

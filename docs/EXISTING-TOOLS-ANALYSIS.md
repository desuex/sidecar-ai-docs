# Existing Tools Analysis

---

## 1. Purpose of This Document

This document evaluates existing documentation systems to determine:

* Their architectural model
* Their storage approach
* Their anchoring mechanisms
* Their indexing capabilities
* Their extensibility
* Their suitability as a base for a refactor-resistant, AI-oriented documentation substrate

Tools evaluated:

* Doxygen
* Sphinx
* JSDoc
* DocFX

This analysis is structural, not aesthetic.

---

## 2. Evaluation Criteria

Each tool is evaluated against the following dimensions:

1. Documentation Storage Model
2. Indexing Model
3. Symbol Identity Model
4. Cross-Reference System
5. Refactor Resistance
6. Sidecar Support
7. AI Integration Potential
8. Queryability
9. Extensibility
10. Suitability as Core Substrate

---

## 3. High-Level Comparison

| Feature            | Doxygen | Sphinx | JSDoc   | DocFX    | Target System |
| ------------------ | ------- | ------ | ------- | -------- | ------------- |
| Inline-doc based   | Yes     | Often  | Yes     | Optional | No            |
| Sidecar capable    | Partial | Yes    | Limited | Yes      | Yes           |
| Symbol-level UID   | Weak    | Weak   | Weak    | Moderate | Strong        |
| AST-driven         | Yes     | No     | Partial | Yes      | Yes           |
| Refactor-resistant | No      | No     | No      | Limited  | Yes           |
| Persistent index   | No      | No     | No      | Partial  | Yes           |
| Machine-queryable  | No      | No     | No      | Limited  | Yes           |
| LSP-based          | No      | No     | No      | No       | Yes           |
| AI-ready substrate | No      | No     | No      | No       | Yes           |

---

## 4. Doxygen

### Architecture

* Parses source files.
* Extracts inline comments.
* Builds documentation model.
* Generates static HTML/LaTeX/XML.

### Storage Model

* Documentation lives inside code comments.
* Limited external documentation support.

### Strengths

* Strong C/C++ parsing.
* Good symbol extraction.
* Generates cross-reference links.
* Mature ecosystem.

### Limitations

* Inline-doc dependency.
* Anchored to line numbers and text positions.
* No stable UID system.
* No incremental persistent index.
* No external query interface.
* Not designed for AI integration.

### Suitability

Good as a renderer.

Not suitable as semantic substrate.

---

## 5. Sphinx

### Architecture

* Markdown/reStructuredText source.
* Domain extensions (Python, C, etc.).
* Static site generation.

### Storage Model

* Documentation-first.
* Code integration via extensions (autodoc).

### Strengths

* Strong documentation authoring model.
* Rich cross-referencing.
* Plugin ecosystem.
* Clean sidecar support.

### Limitations

* Code is secondary to documentation.
* No persistent symbol graph.
* No UID stability.
* No refactor-aware anchoring.
* No structured query API.

### Suitability

Strong as a publishing layer.

Not suitable as indexing substrate.

---

## 6. JSDoc

### Architecture

* Inline comment extraction.
* JavaScript-specific.
* Static site output.

### Storage Model

* Comment-driven.
* Embedded documentation.

### Strengths

* Easy to adopt.
* Strong JS ecosystem integration.

### Limitations

* No persistent index.
* Weak symbol identity model.
* No refactor resilience.
* No structured query engine.
* No AI-oriented design.

### Suitability

Developer convenience tool.

Not infrastructure.

---

## 7. DocFX

### Architecture

* Extracts API metadata.
* Consumes Markdown.
* Generates static documentation.
* Supports .NET ecosystem strongly.

### Storage Model

* Hybrid:

  * Metadata extraction
  * Markdown sidecar docs
* Supports cross-reference maps.

### Strengths

* UID-style cross references (for API).
* XRef maps.
* External Markdown support.
* Decoupled rendering.

### Limitations

* UID not designed for refactor rebasing.
* No AST-diff anchoring.
* No persistent interactive index.
* No query engine.
* No AI substrate design.

### Suitability

Closest conceptual ancestor.

Still rendering-oriented.

---

## 8. Core Observations

### 8.1 All Tools Are Rendering-Oriented

All evaluated tools focus on:

* Generating documentation output.
* Human browsing experience.
* Static content.

None focus on:

* Persistent semantic graph.
* Structured query engine.
* Refactor resilience.
* AI-first design.

---

### 8.2 Inline Documentation Is the Dominant Pattern

Most tools assume:

* Documentation inside code.
* Comments are the source of truth.

This causes:

* Merge conflicts.
* Noise in diffs.
* Reduced clarity.
* Poor separation of concerns.

---

### 8.3 UID Systems Are Weak or Absent

DocFX partially introduces UID concepts.

However:

* UID stability across refactors is not guaranteed.
* No diff-aware rebasing exists.
* No structural fingerprinting.

---

### 8.4 No Tool Provides a Query API

None provide:

* CLI query engine
* Structured JSON responses
* Partial field selection
* Ranked references
* Impact analysis

All assume browsing, not querying.

---

### 8.5 No Tool Is AI-Native

AI support today means:

* Dumping generated HTML into LLM context.
* Or feeding raw source code.

This is inefficient.

No tool exposes:

* Symbol-level structured retrieval
* Bounded snippet extraction
* Token-efficient contracts
* MCP-compatible server

---

## 9. Can Any Tool Be Extended?

### Doxygen

Extending into a semantic index would require:

* Replacing output layer
* Replacing anchor model
* Replacing storage layer
* Adding UID system
* Adding query engine

This equals rewriting core.

---

### Sphinx

Could be used as:

* Rendering target
* Documentation authoring frontend

But not indexing backbone.

---

### JSDoc

Too limited architecturally.

---

### DocFX

Most promising base for:

* UID-style referencing
* Metadata extraction

However still lacks:

* Refactor-resilient anchoring
* Persistent semantic index
* AI-native design

Would require major architectural augmentation.

---

## 10. Strategic Conclusion

Existing tools solve:

* Presentation
* Publishing
* Developer portal needs

They do not solve:

* Semantic indexing
* UID-first documentation
* Refactor-resilient anchors
* Structured query layer
* AI substrate architecture

Therefore:

This project must build a new core.

Rendering layers can reuse existing tools.

But indexing and identity cannot be delegated.

---

## 11. Design Implication

The system should:

* Not attempt to compete as a static site generator.
* Focus on semantic infrastructure.
* Optionally export to DocFX/Sphinx.

Core priority:

Structured, refactor-aware documentation graph.

Everything else is secondary.

Roadmap

⸻

1. Purpose

This roadmap defines the phased development plan for the project.

Goals:
	•	Refactor-safe documentation binding
	•	Deterministic symbol identity
	•	Token-efficient AI interaction
	•	IDE integration
	•	Production-grade stability

The roadmap is incremental.

Each phase must deliver working, testable functionality.

No speculative features without foundation.

⸻

2. Phase 0 — Foundation (Research & Spec Finalization)

Status: ✅ Complete (Spec Seed)

Deliverables:
	•	Architecture overview
	•	UID model
	•	Anchoring model
	•	AST diff model
	•	Storage specification
	•	MCP specification
	•	CLI specification
	•	IDE plugin specifications
	•	Security model
	•	Evaluation plan
	•	Test vectors

Exit Criteria:
	•	Spec reviewed
	•	Determinism guarantees defined
	•	UID version locked (v1)

⸻

3. Phase 1 — Minimal Viable Index

Goal: Deterministic symbol identity + basic querying.

3.1 Core Features
	•	Tree-sitter integration
	•	Symbol extraction
	•	UID generation (v1)
	•	Fingerprint computation
	•	SQLite storage backend
	•	Reference extraction (intra-file minimum)
	•	CLI: index, symbol, search
	•	Basic MCP server (read-only)

3.2 Excluded
	•	Anchoring
	•	AST diff
	•	Rebinding
	•	Ranking sophistication
	•	IDE integration

3.3 Exit Criteria
	•	UID determinism validated
	•	Formatting changes preserve UID
	•	Deterministic search results
	•	CLI JSON output stable
	•	Initial test vectors pass

⸻

4. Phase 2 — Reference Graph & Ranking

Goal: Cross-file references + deterministic ranking.

4.1 Features
	•	Cross-file reference resolution
	•	Reference count normalization
	•	Deterministic ranking implementation
	•	Pagination support
	•	Relevance scoring
	•	MCP tools:
	•	find_references
	•	search_symbols (ranked)
	•	Ranking test vectors

4.2 Exit Criteria
	•	Ranking deterministic across runs
	•	Pagination stable
	•	Reference sampling bounded
	•	Token economy enforced
	•	Large repo test passes (≥100k symbols)

⸻

5. Phase 3 — Sidecar Documentation

Goal: Documentation stored separately and bound to symbols.

5.1 Features
	•	Sidecar directory detection
	•	YAML metadata parsing (safe mode)
	•	Anchor binding to symbol UID
	•	Documentation retrieval
	•	CLI:
	•	doc
	•	unresolved
	•	coverage
	•	MCP:
	•	get_documentation
	•	list_docs

5.2 Excluded
	•	Anchor rebinding
	•	AST diff

5.3 Exit Criteria
	•	Documentation attaches correctly
	•	Unresolved docs detected
	•	Coverage metrics available
	•	Sidecar validation passes security tests

⸻

6. Phase 4 — AST Diff & Refactor Resilience

Goal: Refactor-safe documentation.

6.1 Features
	•	AST diff engine
	•	Rename detection
	•	Move detection
	•	Similarity scoring
	•	Anchor rebinding
	•	Migration event logging
	•	Anchor confidence scoring
	•	CLI:
	•	validate
	•	rebind
	•	MCP:
	•	get_migrations

6.2 Test Coverage
	•	Rename vectors
	•	Move vectors
	•	Extract method vectors
	•	Delete symbol vectors

6.3 Exit Criteria
	•	≥90% correct rebind on rename/move
	•	0 silent incorrect rebinds
	•	Anchor confidence accurate
	•	All anchor test vectors pass

⸻

7. Phase 5 — Selector Model

Goal: Attach documentation to non-symbol AST nodes.

7.1 Features
	•	Selector DSL implementation
	•	Subtree hash support
	•	child_index fallback
	•	Confidence scoring
	•	Selector rebinding via AST diff

7.2 Exit Criteria
	•	Selectors survive formatting
	•	Selectors survive sibling reorder
	•	Confidence scoring correct
	•	No silent misbinding

⸻

8. Phase 6 — MCP Full Implementation

Goal: AI-ready interface.

8.1 Features
	•	Full MCP tool suite
	•	Field-level selection
	•	Token-aware truncation
	•	Pagination enforcement
	•	Deterministic JSON responses
	•	Schema versioning

8.2 Exit Criteria
	•	Deterministic responses
	•	Token economy metrics validated
	•	No implicit expansion
	•	Performance targets met

⸻

9. Phase 7 — VS Code Extension

Goal: First IDE integration.

9.1 Features
	•	Hover documentation
	•	Coverage markers
	•	Anchor health warnings
	•	Sidecar open command
	•	MCP connection handling
	•	Configuration panel

9.2 Exit Criteria
	•	Hover <100ms cached
	•	No UI blocking
	•	Stable behavior under reindex
	•	Deterministic interaction

⸻

10. Phase 8 — JetBrains Plugin

Goal: Second IDE integration.

10.1 Features
	•	PSI integration
	•	Hover documentation
	•	Line markers
	•	Tool window explorer
	•	Refactor event refresh

10.2 Exit Criteria
	•	No EDT blocking
	•	Stable refactor handling
	•	Plugin security validation
	•	Performance benchmarks met

⸻

11. Phase 9 — Performance & Scalability Hardening

Goal: Production-grade performance.

11.1 Features
	•	Index optimization
	•	Incremental indexing
	•	Large repo stress testing
	•	Memory profiling
	•	Query caching
	•	Query cost metrics

11.2 Exit Criteria
	•	<100ms typical query
	•	<300ms heavy query
	•	Near-linear scaling
	•	No O(n²) regressions

⸻

12. Phase 10 — Security Hardening

Goal: Production security posture.

12.1 Features
	•	Strict UID validation
	•	Path traversal protection
	•	YAML schema validation
	•	Input size limits
	•	Rate limiting
	•	Malformed input rejection

12.2 Exit Criteria
	•	All adversarial test vectors pass
	•	No injection vulnerabilities
	•	No unsafe file access
	•	No unbounded query behavior

⸻

13. Phase 11 — Embedding Layer (Optional)

Goal: Optional semantic enhancement.

13.1 Features
	•	Embedding index
	•	Semantic ranking overlay
	•	Opt-in only
	•	Deterministic fallback

13.2 Constraints
	•	Must not break lexical determinism
	•	Must not alter UID
	•	Must not increase default token usage

Optional enhancement only.

⸻

14. Phase 12 — CI & Ecosystem Tooling

Goal: Integration readiness.

14.1 Features
	•	CI indexing mode
	•	Index snapshot diff tool
	•	Migration report tool
	•	Coverage badge generation
	•	Documentation health report

14.2 Exit Criteria
	•	Reproducible index in CI
	•	Stable CLI JSON output
	•	Regression test automation complete

⸻

15. Versioning Plan

v0.x → Rapid iteration
v1.0 → Stable UID v1, stable MCP v1
v2.0 → Breaking UID or schema change (if needed)

Breaking changes require:
	•	Migration tool
	•	Version bump
	•	Explicit documentation

⸻

16. Non-Goals (Short-Term)

Not in immediate roadmap:
	•	Automatic documentation generation
	•	Cloud-hosted MCP
	•	Multi-repo linking
	•	Runtime profiling integration
	•	Language server replacement
	•	AI training integration

Foundation first.

⸻

17. Long-Term Vision (Post v1)
	•	Cross-repo symbol linking
	•	Large-scale documentation graph visualization
	•	Impact simulation for refactors
	•	Concept-level architecture navigation
	•	Semantic compression for AI workflows
	•	IDE-native graph exploration
	•	Distributed indexing

Must preserve:
	•	Determinism
	•	Token economy
	•	Refactor resilience

⸻

18. Success Criteria

Project successful if:
	•	Documentation survives refactors
	•	UID stable under formatting
	•	Ranking deterministic
	•	Token reduction ≥60% vs naive context
	•	IDE integration seamless
	•	Security constraints enforced
	•	Large repo performance acceptable
	•	Developer trust achieved

⸻

19. Philosophy

Roadmap prioritizes:
	1.	Identity stability
	2.	Determinism
	3.	Refactor resilience
	4.	Token economy
	5.	Performance
	6.	UX integration
	7.	Optional semantic enhancements

No shortcuts around core identity model.

Everything builds on UID determinism.

Contributing Guide

⸻

1. Purpose

This document defines how to contribute to Sidecar AI Code Documentation.

This project values:
	•	Determinism
	•	Refactor resilience
	•	Token economy
	•	Structural correctness
	•	Security-first design
	•	Explicit architectural reasoning

Contributions must align with these principles.

⸻

2. Before You Start

Before submitting code:
	•	Read README.md
	•	Read docs/ARCHITECTURE-OVERVIEW.md
	•	Read docs/UID-AND-XREF-MODEL.md
	•	Read docs/ANCHORING-SPEC.md
	•	Read docs/TOKEN-ECONOMY-STRATEGY.md
	•	Read docs/SECURITY-MODEL.md

If your proposal conflicts with determinism or refactor resilience, it will likely be rejected.

⸻

3. Types of Contributions

We welcome:
	•	Bug fixes
	•	Determinism improvements
	•	Performance improvements
	•	Security improvements
	•	Additional test vectors
	•	Documentation clarity improvements
	•	New language adapters (Tree-sitter based)
	•	IDE integration improvements
	•	CLI improvements
	•	MCP tooling improvements

We are cautious about:
	•	Heuristic-heavy features
	•	Non-deterministic behavior
	•	Implicit expansions
	•	Hidden ranking logic
	•	AI-driven dynamic behavior without structure

⸻

4. Contribution Workflow
	1.	Fork repository.
	2.	Create feature branch.
	3.	Write tests first (if applicable).
	4.	Implement feature.
	5.	Run full test suite.
	6.	Ensure deterministic output.
	7.	Submit pull request.

Pull requests must:
	•	Explain motivation
	•	Reference related issue (if any)
	•	Include tests
	•	Preserve determinism
	•	Not introduce token inflation
	•	Not weaken security posture

⸻

5. Coding Standards

5.1 Determinism First

Never introduce:
	•	Randomness
	•	Time-based ordering
	•	Non-stable iteration over maps
	•	Implicit reordering
	•	Hidden ranking adjustments

All outputs must be reproducible.

⸻

5.2 No Hidden Magic

Avoid:
	•	Implicit graph traversal
	•	Implicit field expansion
	•	Automatic full-document responses
	•	Heuristic overrides without versioning

Explicit parameters are required.

⸻

5.3 Bounded Behavior

All queries must:
	•	Enforce limits
	•	Enforce depth bounds
	•	Enforce size bounds
	•	Enforce timeouts

No unbounded operations allowed.

⸻

5.4 Security Discipline

All new code must:
	•	Validate inputs
	•	Use safe parsers
	•	Avoid code execution
	•	Avoid dynamic evaluation
	•	Prevent path traversal
	•	Reject malformed UIDs
	•	Reject oversized input

Security regressions block merge.

⸻

6. Test Requirements

All functional changes must include:
	•	Unit tests
	•	Snapshot tests (if output-based)
	•	Determinism validation
	•	Negative test cases (if applicable)

If change affects:
	•	UID logic → include UID stability tests
	•	Ranking → include ranking stability tests
	•	Anchoring → include rebind test vectors
	•	MCP → include JSON schema tests
	•	CLI → include JSON output tests

All tests must pass in CI.

⸻

7. Test Vectors

If adding new behavior:
	•	Add test vectors in tests/vectors/
	•	Include before/after state
	•	Include expected migration events
	•	Include expected anchor confidence

Test vectors are required for:
	•	AST diff changes
	•	Anchor rebinding changes
	•	UID generation changes
	•	Selector behavior changes

⸻

8. Documentation Contributions

Documentation must:
	•	Be precise
	•	Avoid marketing language
	•	Avoid vague claims
	•	Reflect actual implementation
	•	Preserve architectural consistency

All spec changes must:
	•	Be versioned
	•	Be reflected in ROADMAP if relevant
	•	Be reflected in GLOSSARY if new terms introduced

⸻

9. Performance Considerations

All contributions must consider:
	•	Query latency
	•	Indexing latency
	•	Memory footprint
	•	Token output size

Performance regressions must be justified.

⸻

10. Backward Compatibility

Breaking changes require:
	•	Major version bump
	•	Migration path
	•	Updated compatibility documentation
	•	Explicit changelog entry

Never introduce silent breaking changes.

⸻

11. Pull Request Review Criteria

PRs will be reviewed for:
	•	Correctness
	•	Determinism
	•	Test coverage
	•	Security posture
	•	Token economy adherence
	•	Architectural alignment
	•	Code clarity
	•	Backward compatibility

PRs may be rejected if:
	•	They introduce hidden heuristics
	•	They reduce determinism
	•	They increase token output unnecessarily
	•	They weaken security boundaries
	•	They violate UID stability guarantees

⸻

12. Issue Reporting

When filing an issue:

Include:
	•	Environment
	•	Version
	•	Minimal reproduction
	•	Expected behavior
	•	Actual behavior
	•	Logs (if safe)

Avoid vague reports like “it doesn’t work.”

⸻

13. Design Proposals

For major changes:
	1.	Open discussion issue.
	2.	Provide structured proposal:
	•	Problem
	•	Proposed solution
	•	Determinism impact
	•	Security impact
	•	Token economy impact
	•	Migration impact
	3.	Await feedback before implementation.

Architecture changes must be deliberate.

⸻

14. Commit Message Guidelines

Use structured commit messages:

type(scope): short summary

Detailed explanation if necessary.

Types:
	•	feat
	•	fix
	•	refactor
	•	perf
	•	test
	•	docs
	•	security
	•	chore

Avoid ambiguous commit messages.

⸻

15. Maintainer Authority

Maintainers may:
	•	Request revisions
	•	Reject contributions
	•	Close proposals
	•	Enforce standards
	•	Rework PRs

Decisions prioritize long-term architectural integrity.

⸻

16. Non-Goals for Contributors

Do not submit:
	•	Auto-documentation generators (yet)
	•	AI-generated heuristic binding
	•	Runtime instrumentation hooks
	•	Hidden analytics
	•	Telemetry without discussion
	•	Non-deterministic ranking

⸻

17. Philosophy

This project is:
	•	Structure-first
	•	Deterministic
	•	Refactor-aware
	•	Token-efficient
	•	Security-conscious
	•	Explicit over implicit

If your change weakens any of these properties, reconsider it.

⸻

Thank you for contributing.
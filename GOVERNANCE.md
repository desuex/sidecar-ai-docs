Governance Model

⸻

1. Purpose

This document defines the governance structure of the Sidecar AI Code Documentation project.

Governance exists to:
	•	Preserve architectural integrity
	•	Protect determinism guarantees
	•	Maintain security posture
	•	Avoid feature drift
	•	Ensure long-term sustainability
	•	Provide transparent decision-making

This project prioritizes structural correctness over feature velocity.

⸻

2. Project Structure

The project is organized around:
	•	Core Maintainers
	•	Contributors
	•	Reviewers
	•	Users

Authority is scoped, not arbitrary.

⸻

3. Core Maintainers

Core Maintainers are responsible for:
	•	Architectural decisions
	•	UID model stability
	•	Schema versioning
	•	MCP contract stability
	•	Ranking determinism
	•	Security posture
	•	Breaking changes approval
	•	Roadmap direction

Maintainers must:
	•	Understand the full architecture
	•	Protect determinism
	•	Prioritize refactor resilience
	•	Enforce token economy discipline

Maintainers have final decision authority.

⸻

4. Maintainer Responsibilities

Maintainers must:
	•	Review pull requests
	•	Enforce coding standards
	•	Enforce determinism
	•	Ensure test coverage
	•	Prevent architectural erosion
	•	Approve version releases
	•	Approve breaking changes
	•	Respond to security reports
	•	Maintain roadmap coherence

Maintainers must act in good faith.

⸻

5. Contributors

Contributors may:
	•	Submit pull requests
	•	Propose design changes
	•	Submit test vectors
	•	Improve documentation
	•	Report bugs

Contributors must:
	•	Follow CONTRIBUTING.md
	•	Preserve determinism
	•	Respect Code of Conduct
	•	Avoid introducing hidden heuristics
	•	Avoid breaking compatibility without versioning

Contribution does not automatically grant governance authority.

⸻

6. Decision-Making Process

6.1 Minor Changes

Minor changes (bug fixes, documentation updates, test additions):
	•	Require one maintainer approval
	•	Must pass CI
	•	Must not break determinism

⸻

6.2 Architectural Changes

Architectural changes require:
	•	Design discussion issue
	•	Explicit proposal
	•	Security impact analysis
	•	Determinism impact analysis
	•	Migration plan (if needed)
	•	Maintainer consensus

Consensus preferred, but majority of maintainers decides.

⸻

6.3 Breaking Changes

Breaking changes require:
	•	Major version bump
	•	Migration plan
	•	Documentation update
	•	Roadmap update
	•	Explicit approval from maintainers

Breaking changes must not be merged silently.

⸻

7. Release Management

Releases must:
	•	Be versioned semantically
	•	Include changelog
	•	Include migration notes (if needed)
	•	Pass all tests
	•	Pass determinism checks
	•	Pass security checks

Maintainers are responsible for tagging releases.

⸻

8. Security Governance

Security reports:
	•	Must be handled privately
	•	Must be triaged promptly
	•	Must result in patch release
	•	Must include regression test
	•	Must be documented in changelog

Security fixes take priority over feature development.

⸻

9. Architectural Guardrails

The following are protected principles:
	•	UID determinism
	•	Refactor resilience
	•	Explicit over implicit behavior
	•	No hidden ranking logic
	•	No unbounded queries
	•	No silent anchor rebinding
	•	No automatic code execution
	•	No undocumented breaking changes

Any proposal violating these principles will be rejected.

⸻

10. Maintainer Selection

New maintainers may be invited based on:
	•	Long-term contribution
	•	Deep architectural understanding
	•	Commitment to determinism
	•	Security awareness
	•	Constructive participation
	•	Demonstrated reliability

Maintainer status is not automatic.

Maintainers may be removed by consensus if:
	•	Repeated governance violations occur
	•	Security negligence occurs
	•	Project integrity is compromised

⸻

11. Conflict Resolution

If disagreements arise:
	1.	Discuss in issue thread.
	2.	Present structured argument.
	3.	Evaluate impact on:
	•	Determinism
	•	Security
	•	Token economy
	•	Compatibility
	4.	Maintainers decide.

Escalation beyond maintainers is not defined; project authority rests internally.

⸻

12. Transparency

The following must remain public:
	•	Roadmap
	•	Architectural specifications
	•	Breaking change rationale
	•	Version history
	•	Migration plans
	•	Governance model

Private communication reserved for:
	•	Security issues
	•	Sensitive moderation matters

⸻

13. Non-Goals of Governance

Governance does not:
	•	Guarantee consensus for every feature
	•	Prioritize popularity over correctness
	•	Allow architecture to drift for convenience
	•	Automatically merge large feature sets
	•	Accept non-deterministic shortcuts

This project values correctness over speed.

⸻

14. Evolution of Governance

Governance model may evolve, but changes must:
	•	Be proposed publicly
	•	Include rationale
	•	Preserve core principles
	•	Be approved by maintainers

Governance changes must be versioned in documentation.

⸻

15. Project Philosophy

This project is built on:
	•	Structural identity
	•	Deterministic computation
	•	Refactor-aware binding
	•	Token-efficient interaction
	•	Explicit design contracts
	•	Security discipline

Governance exists to protect these principles.

Without governance, architecture degrades.

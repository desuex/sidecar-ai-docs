# REFLECTION.md

## 0. Purpose

This document defines a core rule of the project:

> **Sidecar must document itself using the Sidecar format.**

The repository is not only an implementation of the system.
It is the **reference implementation of how Sidecar documentation should look, behave, evolve, and survive refactors**.

This repo is the gold standard.

---

## 1. Self-Hosting Principle

Sidecar must be:

* indexed by itself
* documented via sidecar docs
* navigable via its own MCP
* refactor-tested against its own anchors
* used as the primary testbed for anchor stability

If Sidecar cannot reliably document Sidecar, it is not ready for production use.

---

## 2. Documentation as First-Class Artifact

Documentation in this project is:

* structured
* sidecar-based
* UID-bound
* refactor-aware
* versioned
* testable

It is not optional.

Every public-facing symbol in the core should eventually have:

* sidecar documentation entry
* summary mode
* full mode
* cross-reference links (UID-based)
* explicit anchor confidence

---

## 3. What Must Be Documented

At minimum:

### 3.1 Core Identity Layer

* UID generation logic
* fingerprint normalization rules
* canonicalization rules
* ranking logic
* pagination guarantees

### 3.2 Storage Layer

* schema versioning
* migration strategy
* ordering guarantees

### 3.3 Parsing Layer

* language adapters
* symbol extraction rules
* reference extraction rules

### 3.4 MCP Layer

* tool contracts
* response guarantees
* truncation semantics
* determinism guarantees

### 3.5 CLI Surface

* commands
* exit codes
* JSON shapes

---

## 4. Documentation Workflow (Required Discipline)

When changing code:

1. Update tests.
2. Update documentation (sidecar entry).
3. Run index.
4. Validate anchors.
5. Ensure no documentation is orphaned.
6. Ensure coverage does not regress.

Code changes without doc changes are incomplete.

---

## 5. Documentation Coverage as a Metric

The project must expose:

* documented symbol percentage
* undocumented public symbol list
* broken anchor list
* low-confidence anchor list

These are not vanity metrics.

They are integrity metrics.

---

## 6. Refactor Resilience Testing on Itself

Sidecar must:

* survive renames in its own code
* survive file moves
* survive method extraction
* detect broken anchors
* rebind correctly where possible
* report confidence

We test rebinding on this repository first.

---

## 7. Sidecar Format as Canonical Example

The documentation in this repository should demonstrate:

* proper YAML front matter
* proper UID targeting
* selector usage (when implemented)
* summary vs full sections
* cross-reference linking via UID
* anchor confidence tracking

This repository is the documentation style guide.

---

## 8. “No Silent Drift” Policy

It is unacceptable for:

* code to change without doc update
* doc to reference non-existing UID
* UID to change without migration note
* ranking logic to change without doc update

Drift must be visible and actionable.

---

## 9. Golden Reference Repository

This repository serves as:

* integration test
* documentation coverage benchmark
* refactor stress test
* token economy benchmark
* performance benchmark
* migration testbed

Future adopters should be able to:

* inspect this repo
* run Sidecar on it
* see clean outputs
* see healthy anchors
* see high coverage

---

## 10. Adoption Signal

If this repository demonstrates:

* high coverage
* stable anchors across refactors
* low token outputs via MCP
* clean deterministic outputs

Then it proves:

Sidecar works in the real world.

---

## 11. Documentation Debt Is Technical Debt

Undocumented core logic:

* increases cognitive load
* increases token usage
* increases refactor risk
* increases onboarding cost

Documentation debt is not cosmetic debt.

It is structural debt.

---

## 12. The Gold Standard Commitment

This project commits to:

* documenting itself first
* maintaining documentation rigorously
* using Sidecar format internally
* treating documentation regressions as real regressions

Sidecar must be:

> The best documented project using Sidecar.

Anything less undermines credibility.

---

## 13. Long-Term Vision

Eventually:

* Sidecar’s own documentation becomes a canonical example.
* Adopters copy structure and conventions.
* MCP output from this repo becomes example payload in docs.
* Refactor resilience is demonstrated live through commit history.

The project becomes both:

* tool
* and proof

---

## 14. Final Rule

If Sidecar cannot reliably document and maintain itself:

It is not ready for others.
# AI-DOCS-WORKFLOW.md

## 0. Purpose

This document defines how AI agents contribute to Sidecar documentation safely, deterministically, and with measurable quality.

Goal:

* Use local or remote LLMs to accelerate documentation
* Prevent hallucinated or low-quality docs
* Maintain structural integrity
* Keep documentation refactor-resilient
* Preserve token economy

AI-generated documentation must be **controlled, verifiable, and reviewable**.

---

## 1. Core Principle

> AI agents may generate documentation, but never define truth.

Truth comes from:

* Source code
* UID-bound structure
* Reference graph
* Deterministic queries

AI may summarize and explain — not invent.

---

## 2. Roles

### 2.1 Sidecar (Index + Structure)

Sidecar provides:

* UID identity
* Symbol metadata
* References
* Usage frequency
* Deterministic ordering
* Coverage metrics
* Anchor validation
* Staleness detection

Sidecar is the authority on structure.

---

### 2.2 AI Agent

AI agents may:

* Generate documentation drafts
* Suggest summaries
* Extract invariants from code
* Propose usage examples
* Suggest clarifications

AI agents must:

* Use bounded queries (MCP)
* Cite UID and references
* Respect output format
* Not modify anchors directly
* Mark documentation as `draft`

---

### 2.3 Human Reviewer

Humans:

* Approve or reject drafts
* Upgrade `draft` → `verified`
* Fix incorrect explanations
* Validate invariants and side effects
* Make judgment calls

AI accelerates; humans validate.

---

## 3. Documentation Status Model

Each documentation entry must include a status field:

* `draft` — AI-generated, not yet reviewed
* `reviewed` — human reviewed but not formally verified
* `verified` — validated against code behavior
* `stale` — code changed since last validation
* `orphaned` — target UID missing

Status must be stored in metadata and validated by Sidecar.

---

## 4. AI Documentation Pipeline

### Step 1 — Identify Targets

Use MCP to measure documentation coverage and fetch a bounded backlog:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "coverage_metrics",
  "params": {
    "public_only": true,
    "scan_limit": 5000
  }
}
```

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "detect_undocumented_symbols",
  "params": {
    "public_only": true,
    "scan_limit": 5000,
    "limit": 100,
    "offset": 0
  }
}
```

Select:

* undocumented public symbols
* high-usage undocumented symbols
* stale documentation
* low-confidence anchors

Never generate docs blindly for the entire repo.

---

### Step 2 — Gather Context (Bounded)

For each symbol:

1. `get_symbol(uid)`
2. `find_references(uid, limit=N)`
3. Read:

   * definition node
   * 1–3 usage sites
   * small code window only (bounded)

Never read entire repository.

---

### Step 3 — Generate Structured Draft

Draft must follow this minimal structure:

```markdown
---
uid: <target_uid>
status: draft
---

## Summary
<1–3 concise sentences>

## Behavior
- What it does
- Inputs / outputs
- Side effects (only if observed in code)

## Constraints
- Preconditions
- Invariants
- Edge cases

## Usage
- Typical call patterns (based on real references)

## Related
- UID links to related symbols
```

No essays. No speculative commentary.

---

### Step 4 — Validate

Run:

```bash
sidecar validate
```

Must confirm:

* UID exists
* Anchor matches
* No traversal errors
* No malformed metadata
* Documentation linked correctly

---

### Step 5 — Submit PR

AI-generated documentation must:

* Clearly state it is AI-generated
* Include referenced UIDs
* Pass CI checks

Humans review before merge.

---

## 5. Hallucination Prevention Rules

AI must not:

* Mention parameters not in signature
* Describe behavior not observable in code
* Invent thrown errors
* Invent side effects
* Reference non-existent UIDs
* Speculate about business logic

If uncertain, AI must state:

> “Behavior inferred from usage patterns; needs review.”

---

## 6. Evidence Requirement

Every claim should be grounded in one of:

* Symbol signature
* Observed references
* Literal code behavior
* Inline comments in code

Agents should rely on:

* `find_references`
* bounded code snippets
* structural metadata

Sidecar enables evidence-based summarization.

---

## 7. Documentation Quality Metrics

Sidecar must expose:

* Documentation coverage %
* Undocumented public symbols
* Stale documentation count
* Broken anchor count
* Low-confidence anchor count
* Draft vs verified ratio

These are integrity metrics, not vanity metrics.

---

## 8. Refactor Awareness

When code changes:

* Affected documentation becomes `stale`
* Anchor confidence may drop
* Validation flags must surface issues

AI agents may regenerate drafts for stale entries.

Human review required for re-verification.

---

## 9. Token Economy Discipline

AI documentation must:

* Use bounded context
* Avoid full-file ingestion
* Avoid repo-wide summarization
* Avoid duplication across entries
* Prefer linking via UID over repeating content

Cross-referencing > repetition.

---

## 10. Self-Hosting Rule

Sidecar must:

* Document itself via this workflow
* Use its own MCP to generate drafts
* Validate its own anchors
* Maintain high coverage

This repository is the gold standard example.

---

## 11. Anti-Patterns

Never allow:

* Auto-generating docs for entire repo without prioritization
* Overwriting human-verified documentation automatically
* Storing large prose blobs without structure
* Allowing stale documentation to persist silently
* Treating AI output as authoritative

---

## 12. Long-Term Vision

In a mature Sidecar ecosystem:

* AI agents propose drafts continuously
* CI validates structure and anchors
* Humans focus on correctness, not typing
* Documentation stays aligned with code
* Refactors do not silently break docs
* Token usage remains bounded and predictable

Sidecar becomes a documentation substrate, not a doc generator.

---

## 13. Final Rule

AI assists documentation.

Sidecar enforces structure.

Humans define truth.

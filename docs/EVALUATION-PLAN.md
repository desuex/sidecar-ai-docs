# Evaluation Plan

---

## 1. Purpose

This document defines how the system will be evaluated.

Evaluation must measure:

* Refactor resilience
* Anchor stability
* UID determinism
* Token efficiency
* Query latency
* Ranking correctness
* Scalability
* Developer workflow impact

Evaluation must be:

* Reproducible
* Automated where possible
* Deterministic
* Measurable
* Quantitative

Subjective impressions are insufficient.

---

## 2. Evaluation Categories

The system will be evaluated across:

1. Identity Stability
2. Anchoring Resilience
3. AST Diff Accuracy
4. MCP Determinism
5. Token Efficiency
6. Ranking Quality
7. Performance
8. Scalability
9. IDE Integration Responsiveness
10. CLI Reliability
11. Documentation Coverage Tracking

---

## 3. Identity Stability Evaluation

### 3.1 Test Cases

Apply transformations:

* Reformat file
* Reorder methods
* Rename variable
* Rename method
* Move method to another class
* Extract method
* Change signature
* Move file
* Delete symbol

### 3.2 Metrics

* % UID preserved when appropriate
* % UID changed correctly when required
* False-positive UID changes
* False-negative UID stability

### 3.3 Success Criteria

* Formatting-only changes → 100% UID stability
* Method reorder → 100% UID stability
* Rename detection → ≥ 90% correct remap
* Major rewrite → 0 incorrect remap

---

## 4. Anchoring Resilience Evaluation

### 4.1 Test Scenarios

* Rename symbol
* Move symbol
* Extract block
* Delete symbol
* Modify signature
* Split method

### 4.2 Metrics

* Anchor rebind accuracy
* Anchor confidence score accuracy
* False rebind rate
* Unresolved anchor rate

### 4.3 Success Criteria

* ≥ 90% correct rebind for rename/move
* 0 silent incorrect rebinds
* All unresolved anchors surfaced

---

## 5. AST Diff Accuracy Evaluation

### 5.1 Dataset

* Synthetic controlled refactors
* Real-world refactor commits
* Multi-language repositories

### 5.2 Metrics

* Node match precision
* Node match recall
* Move detection accuracy
* Rename detection accuracy
* Split detection accuracy

### 5.3 Performance Metrics

* Diff time per file
* Memory overhead
* Hash collision rate (must be zero)

---

## 6. MCP Determinism Evaluation

### 6.1 Determinism Test

Repeat same query 100 times.

Compare:

* Response content
* Ordering
* Scores
* Pagination boundaries

### 6.2 Success Criteria

* 100% identical results
* Stable ordering
* Stable pagination
* Stable ranking

---

## 7. Token Efficiency Evaluation

### 7.1 Metrics

Measure:

* Average response size (bytes)
* Estimated token count per query
* Full-content expansion rate
* Truncation frequency
* Pagination effectiveness

### 7.2 Comparative Baseline

Compare against:

* Naive full-file dump
* Naive LSP reference dump
* Inline comment-based documentation

### 7.3 Success Criteria

* ≥ 60% token reduction vs naive context dump
* Bounded default queries
* No automatic large graph expansions

---

## 8. Ranking Quality Evaluation

### 8.1 Symbol Search Evaluation

Test queries:

* Exact match
* Partial match
* CamelCase
* Fuzzy typo
* Common name collisions

Metrics:

* Correct result in top 3
* Correct result in top 5
* Ranking stability across runs

Target:

* ≥ 95% correct top-3 for exact match
* ≥ 90% correct top-5 for fuzzy

---

### 8.2 Documentation Search Evaluation

Metrics:

* Title match ranking accuracy
* Concept match ranking accuracy
* False-positive rate
* Stability across runs

---

## 9. Performance Evaluation

### 9.1 Indexing Performance

Metrics:

* Full index time
* Incremental index time
* Memory usage
* CPU usage
* IO overhead

Targets:

* Incremental update under 100ms for single file
* Full index acceptable for project size
* No O(n²) behavior

---

### 9.2 Query Latency

Measure:

* get_symbol latency
* find_references latency
* get_documentation latency
* search_symbols latency

Targets:

* < 100ms typical query
* < 300ms heavy query
* < 1s worst-case large project

---

## 10. Scalability Evaluation

Test on:

* Small project (<10k symbols)
* Medium project (~100k symbols)
* Large project (~1M symbols)
* Monorepo with multi-language

Metrics:

* Query latency growth
* Index size growth
* Memory footprint
* CPU utilization

Scalability must be near-linear.

---

## 11. IDE Responsiveness Evaluation

### 11.1 Hover Latency

Measure:

* Cold hover
* Cached hover
* Under indexing load

Target:

* < 100ms cached
* < 200ms cold

---

### 11.2 Marker Update

Measure:

* Time to refresh after save
* Impact on typing latency

Typing must never lag due to plugin.

---

## 12. CLI Reliability Evaluation

### 12.1 Command Determinism

Repeat CLI commands:

* Compare JSON outputs
* Compare exit codes
* Compare ordering

### 12.2 Script Integration

Test piping:

```text
sidecar symbol <uid> --json | jq .
```

Ensure stable structured output.

---

## 13. Documentation Coverage Metrics

Evaluate:

* % documented public symbols
* Anchor health ratio
* Concept linkage coverage

Track:

* Before/after adoption
* Per-team metrics
* Long-term drift

---

## 14. Regression Testing Strategy

Must include:

* Snapshot tests for MCP output
* Snapshot tests for CLI JSON
* Anchor rebind regression tests
* UID stability tests
* Ranking stability tests

All regressions must fail CI.

---

## 15. Stress Testing

Stress scenarios:

* Rapid file saves
* Large-scale rename
* Massive refactor
* Large documentation corpus
* Corrupted sidecar file
* Corrupted index database

System must fail safely.

---

## 16. Failure Analysis

Track:

* Anchor misbindings
* UID instability
* Ranking anomalies
* Token over-expansion events
* Query timeout events

Failures must be logged and reproducible.

---

## 17. Developer Workflow Evaluation

Conduct:

* Time-to-find-symbol measurement
* Time-to-locate-doc measurement
* Refactor safety confidence survey
* Token usage comparison in AI-assisted workflows

Qualitative metrics must supplement quantitative data.

---

## 18. Acceptance Criteria

System considered production-ready if:

* UID stability ≥ 95% in safe refactors
* Anchor correct rebind ≥ 90%
* No silent misbinding
* MCP deterministic 100%
* Token reduction ≥ 60% vs naive context
* Query latency within target
* IDE hover under 200ms
* No catastrophic data corruption under stress

---

## 19. Continuous Evaluation

Evaluation must be ongoing:

* Integrated into CI
* Versioned test datasets
* Historical performance tracking
* Benchmark repository set
* Regression dashboard

System must not degrade silently over time.

---

## 20. Summary

Evaluation is not optional.

It ensures:

* Identity correctness
* Anchor durability
* Deterministic ranking
* Token efficiency
* Performance guarantees
* Developer trust

A refactor-aware documentation system must prove its resilience.

Measurement precedes confidence.
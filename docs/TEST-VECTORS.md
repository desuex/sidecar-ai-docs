# Test Vectors

---

## 1. Purpose

This document defines canonical test vectors for validating:

* UID determinism
* Symbol extraction
* Reference extraction
* Anchoring stability
* Selector matching
* AST diff and rebase behavior
* Ranking determinism
* MCP/CLI output determinism

Test vectors are:

* Minimal
* Deterministic
* Reproducible
* Language-scoped
* Refactor-focused

They must be runnable in CI.

---

## 2. Test Vector Format

Each test vector includes:

* language
* input files (before)
* expected index snapshot (before)
* transformation (refactor)
* input files (after)
* expected index snapshot (after)
* expected UID remap events
* expected anchor rebind events
* expected confidence values (bounded)
* expected query outputs (MCP/CLI)

Vectors must be stored as plain files inside a test folder.

Recommended layout:

```text id="c7z6g0"
tests/
  vectors/
    ts/
      V001-formatting/
      V002-reorder-methods/
      V003-rename-method/
      ...
    py/
    go/
    cs/
    rs/
```

---

## 3. Snapshot Schema

Expected snapshots should include:

* symbols list (uid, qualified_name, kind, fingerprint)
* references list (from_uid, to_uid, type)
* file hashes
* anchor states
* uid migration events

Snapshots should be JSON with stable ordering.

---

## 4. TypeScript Test Vectors

---

### TS-V001: Formatting Only

**Goal:** UID must remain identical.

Before:

```ts id="gky9f9"
export class CartService {
  calculateTotal(items: number[]): number {
    return items.reduce((a, b) => a + b, 0);
  }
}
```

After (format changed):

```ts id="xv2g9a"
export class CartService{calculateTotal(items:number[]):number{return items.reduce((a,b)=>a+b,0);}}
```

Expected:

* Same symbol UID for `CartService.calculateTotal`
* Same fingerprint
* No migrations
* Anchor confidence remains 1.0

---

### TS-V002: Reorder Methods

Before:

```ts id="51dc1b"
export class A {
  one() { return 1; }
  two() { return 2; }
}
```

After:

```ts id="xn5x1y"
export class A {
  two() { return 2; }
  one() { return 1; }
}
```

Expected:

* Both method UIDs unchanged
* No migrations
* No anchor changes

---

### TS-V003: Rename Method

Before:

```ts id="cfw08o"
export class A {
  calculateTotal(items: number[]) { return items.length; }
}
```

After:

```ts id="v7v8k2"
export class A {
  calcTotal(items: number[]) { return items.length; }
}
```

Expected:

* Old UID invalid
* New UID created
* Migration event reason = "rename"
* Similarity ≥ 0.9
* Anchor rebound to new UID with confidence ≥ 0.9

---

### TS-V004: Move Method to Another Class

Before:

```ts id="81yxt0"
export class A {
  f(x: number) { return x + 1; }
}
export class B {}
```

After:

```ts id="1h4bxi"
export class A {}
export class B {
  f(x: number) { return x + 1; }
}
```

Expected:

* Migration event reason = "move"
* Similarity ≥ 0.9
* Anchors rebound

---

### TS-V005: Extract Method

Before:

```ts id="o4fp9k"
export class A {
  f(x: number) {
    const y = x + 1;
    return y * 2;
  }
}
```

After:

```ts id="wjy2x2"
export class A {
  helper(x: number) { return x + 1; }
  f(x: number) {
    const y = this.helper(x);
    return y * 2;
  }
}
```

Expected:

* `f` UID may remain stable if signature unchanged and fingerprint policy allows minor changes
* New UID for `helper`
* Reference edge from `f` → `helper`
* No incorrect reassignment of docs to helper

---

## 5. Python Test Vectors

---

### PY-V001: Rename Function

Before:

```py id="u9u2yp"
def compute(x: int) -> int:
    return x + 1
```

After:

```py id="p8b4cq"
def compute_next(x: int) -> int:
    return x + 1
```

Expected:

* Migration reason = rename
* Similarity high via AST body similarity
* Anchor rebound

---

### PY-V002: Move Function Between Files

Before: `a.py`

```py id="e1bcnw"
def f():
    return 1
```

After: `b.py`

```py id="99p9y8"
def f():
    return 1
```

Expected:

* File move detected via git diff improves mapping
* UID changes due to module_path change
* Migration reason = move_file
* Anchor rebound with confidence ≥ 0.85

---

## 6. Go Test Vectors

---

### GO-V001: Method Receiver Rename

Before:

```go id="r9v8cz"
type S struct {}
func (s *S) F(x int) int { return x + 1 }
```

After:

```go id="n6b5xy"
type S struct {}
func (self *S) F(x int) int { return x + 1 }
```

Expected:

* Same UID (receiver name ignored)
* No migration

---

## 7. C# Test Vectors

---

### CS-V001: Rename Method

Before:

```cs id="zwu9u8"
public class A {
  public int F(int x) { return x + 1; }
}
```

After:

```cs id="5tq0u2"
public class A {
  public int G(int x) { return x + 1; }
}
```

Expected:

* Migration reason = rename
* Anchor rebound

---

## 8. Rust Test Vectors

---

### RS-V001: Move Function to Module

Before:

```rs id="n2xk8g"
pub fn f(x: i32) -> i32 { x + 1 }
```

After:

```rs id="os0c7m"
pub mod m {
  pub fn f(x: i32) -> i32 { x + 1 }
}
```

Expected:

* Qualified name changes
* Migration reason = move
* Similarity ≥ 0.85
* Anchor rebound

---

## 9. Selector Test Vectors

---

### SEL-V001: Anonymous Closure Attachment

Before:

```ts id="3gkccx"
items.map(x => x + 1)
```

Doc anchor: selector targets arrow_function subtree.

After:

```ts id="7kqg3p"
items.map((x) => x + 1)
```

Expected:

* Selector still matches
* Confidence remains 1.0

---

### SEL-V002: Sibling Reorder

Before:

```ts id="8y0is3"
if (a) { f(); }
if (b) { g(); }
```

Selector anchored to second if-statement via child_index.

After:

```ts id="btg8t4"
if (b) { g(); }
if (a) { f(); }
```

Expected:

* child_index alone would fail
* subtree hash disambiguates
* Selector rebind confidence ≥ 0.85

---

## 10. MCP Determinism Vectors

---

### MCP-V001: search_symbols Determinism

Run query 100 times:

```json id="eyw9ja"
{ "query": "calculateTotal", "limit": 10 }
```

Expected:

* Identical ordering
* Identical relevance scores
* Identical pagination boundaries

---

### MCP-V002: find_references Pagination Stability

Query:

* limit 10, offset 0
* limit 10, offset 10
* limit 20, offset 0

Expected:

* First 20 results equal combined pages
* No reranking

---

## 11. Ranking Vectors

---

### RANK-V001: Name Collision

Symbols:

* A.calculateTotal
* B.calculateTotal
* CartService.calculateTotal

Query:

* "CartService.calculateTotal"

Expected:

* Exact qualified name match ranked #1
* Others ranked below
* Stable ordering

---

### RANK-V002: Fuzzy Typo

Query:

* "calclateTotal"

Expected:

* calculateTotal within top 5
* Deterministic ranking

---

## 12. Anchor Rebase Vectors

---

### ANCH-V001: Rename + Signature Same

Expected:

* Automatic rebind
* Confidence ≥ 0.9
* Migration logged

---

### ANCH-V002: Delete Symbol

Expected:

* Anchor unresolved
* Confidence 0.0
* Doc retained
* Listed in `sidecar unresolved`

---

## 13. Negative / Adversarial Vectors

---

### ADV-V001: Malformed UID Injection

Input:

```json id="qsq28c"
{ "uid": "sym:../../etc/passwd" }
```

Expected:

* MCP rejects as INVALID_UID
* No file access
* No crash

---

### ADV-V002: Oversized Documentation

Sidecar doc > configured max size.

Expected:

* Validation fails
* Doc flagged
* Index not corrupted

---

## 14. Success Criteria Summary

System passes test vectors if:

* UID determinism holds under non-semantic edits
* Correct migrations logged for rename/move
* Anchors rebind correctly above thresholds
* No silent incorrect rebinds
* Selectors survive formatting/reorder
* MCP/CLI outputs deterministic
* Ranking stable and correct
* Security vectors rejected safely

---

## 15. Future Expansion

Add vectors for:

* Generics and overloads
* Interfaces and implementations
* Inheritance and overrides
* Dynamic dispatch patterns
* Cross-repo SCIP linking
* Multi-language monorepo scenarios
* Generated code exclusion rules

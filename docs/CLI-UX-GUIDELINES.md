# CLI UX Guidelines

---

## 1. Purpose

This document defines user experience principles for the CLI.

The CLI must:

* Feel predictable
* Encourage safe operations
* Be efficient for power users
* Be friendly for new users
* Support automation
* Minimize cognitive overhead
* Avoid unnecessary verbosity

UX consistency is critical for trust.

---

## 2. Core UX Principles

The CLI must be:

* Deterministic
* Explicit
* Transparent
* Safe-by-default
* Discoverable
* Scriptable
* Quiet unless requested

The CLI must not:

* Surprise the user
* Perform hidden operations
* Auto-correct silently
* Mask errors
* Overwhelm with noise

---

## 3. Command Design Philosophy

Commands must:

* Use clear verbs
* Avoid ambiguous naming
* Be consistent across categories
* Follow predictable patterns

Good examples:

```text
sidecar symbol <uid>
sidecar refs <uid>
sidecar doc-create
sidecar anchors-validate
```

Avoid:

* Overloaded commands
* Hidden flags
* Context-dependent behavior

---

## 4. Output Modes

CLI must support two modes:

1. Human-readable mode (default)
2. JSON mode (--json)

Human mode:

* Concise
* Structured
* Colorized (optional)
* Aligned columns
* Clear section headers

JSON mode:

* Deterministic
* Machine-friendly
* No color
* No extra formatting

---

## 5. Human Output Guidelines

Human output must:

* Be readable in narrow terminals
* Avoid wrapping large blobs
* Summarize long lists
* Provide totals
* Highlight warnings

Example:

```text
Symbol: CartService.calculateTotal
Kind: method
File: src/services/cart.ts
Signature: (items: Item[]) => number

Documentation:
  doc:cart-calc-overview

References: 42 (showing 10)
```

---

## 6. Error Messaging

Errors must be:

* Clear
* Specific
* Actionable
* Short

Bad:

```text
Error occurred.
```

Good:

```text
Error: UID not found.
Suggestion: Run `sidecar search-symbols calculateTotal`.
```

No stack traces in normal mode.

Verbose mode may include debug info.

---

## 7. Confirmation UX

Destructive commands must:

* Require explicit flag (e.g. --force)
* Or require confirmation prompt

Example:

```text
sidecar reset
```

Response:

```text
This will delete the index database.
Continue? (y/N)
```

Non-interactive mode must require:

```text
--force
```

---

## 8. Pagination UX

When results exceed limit:

* Show summary line
* Indicate total count
* Suggest next command

Example:

```text
References: 120 (showing 20)
Use --limit 50 or --offset 20 to see more.
```

---

## 9. Anchor Confidence UX

Low-confidence anchors must be visible.

Example:

```text
Warning: Anchor confidence 0.72
Symbol may have been renamed.
Run `sidecar anchors-rebind --dry-run`.
```

Do not silently degrade.

---

## 10. Coverage Reporting UX

Coverage output must:

* Show percentage
* Show counts
* Highlight public undocumented symbols

Example:

```text
Documentation Coverage: 63%

Public Symbols: 142
Documented: 89
Undocumented: 53
```

Avoid dumping entire symbol list unless requested.

---

## 11. Verbosity Levels

CLI must support:

```text
--verbose
--quiet
```

Default:

* Balanced output
* Minimal noise

Verbose:

* Show execution time
* Show internal steps
* Show similarity scores

Quiet:

* Output only essential result

---

## 12. Exit Codes UX

Exit codes must be:

* Meaningful
* Stable
* Documented

Examples:

* 0 → success
* 1 → generic error
* 2 → invalid input
* 3 → not found
* 4 → index unavailable

CLI must not mix exit codes randomly.

---

## 13. Discoverability

CLI must support:

```text
sidecar --help
sidecar symbol --help
```

Help output must:

* Be short
* Show examples
* Show common flags
* Not overwhelm with internal details

---

## 14. Help Text Style

Help text must:

* Use imperative verbs
* Provide usage examples
* Show JSON usage
* Mention pagination

Example:

```text
Usage:
  sidecar symbol <uid>

Options:
  --json        Output JSON
  --fields      Select specific fields
```

---

## 15. Performance Feedback

Long-running commands must:

* Show progress indicator
* Or show indexing status

Example:

```text
Indexing 143 files...
Done. (1.2s)
```

Do not spam progress logs unless verbose.

---

## 16. Deterministic Ordering

Lists must:

* Be sorted deterministically
* Use stable ordering
* Avoid time-based ordering

Sorting rules must be documented.

---

## 17. Scriptability

CLI output must:

* Avoid emojis in JSON mode
* Avoid ASCII art
* Avoid terminal-dependent formatting
* Preserve machine-readability

Human mode may use subtle color, but optional.

---

## 18. Security UX

If command rejected due to safeguard:

Example:

```text
Query exceeds maximum result limit (500).
Use --limit or refine your filter.
```

Never expose internal file paths.

Never expose database structure.

---

## 19. Migration UX

When UID remap occurs:

CLI must show:

```text
UID remapped:
  sym:old → sym:new
Similarity: 0.94
```

Must allow:

```text
--dry-run
```

Before applying.

---

## 20. Non-Goals

CLI UX does not:

* Attempt to be conversational
* Replace IDE UI
* Render markdown
* Auto-generate documentation
* Hide complexity at cost of clarity

CLI is explicit.

---

## 21. Summary

CLI UX must be:

Predictable.
Clear.
Safe.
Minimal.
Deterministic.
Scriptable.

It must earn trust by never surprising the user.

Every command must feel controlled and transparent.
# Vision

---

## 1. The Future of Code Comprehension

Software systems are growing faster than human capacity to hold them in working memory.

Developers no longer work with:

* Files
* Classes
* Functions

They work with:

* Graphs
* Dependencies
* Emergent behavior
* Distributed responsibility

Yet our documentation model remains linear.

The vision of this project is to transition documentation from static narrative to structured semantic infrastructure.

---

## 2. Documentation as a Living Graph

The future documentation system must:

* Reflect the structure of code.
* Track semantic relationships.
* Survive refactors.
* Remain queryable at symbol-level resolution.
* Support both humans and AI agents.

Documentation should not merely describe code.

It should model code intent.

---

## 3. From Text to Structured Knowledge

Traditional documentation:

```text
Markdown → Render → Website
```

Future documentation:

```text
Code → Index → UID Graph → Anchored Docs → Structured Queries
```

The emphasis shifts from rendering to resolution.

Rendering becomes optional.

Querying becomes central.

---

## 4. AI as a First-Class Consumer

AI systems will increasingly:

* Review code
* Refactor code
* Generate documentation
* Perform architecture analysis
* Detect inconsistencies

But AI systems are constrained by:

* Context window limits
* Token costs
* Lack of structural awareness
* Noise from irrelevant code

This system exists to make AI operate on:

* Structured symbol graphs
* Ranked references
* Filtered, minimal context slices

Instead of:

* Raw repository dumps

---

## 5. The End of Line-Number Anchoring

Documentation tied to line numbers is brittle.

Refactors break it.

Moves invalidate it.

Formatting destroys it.

The future requires:

* Semantic anchors
* Structural anchors
* Fuzzy reattachment
* Deterministic rebasing

Documentation must survive movement.

---

## 6. UID-First Systems

Everything important must have a stable identity.

Not just:

* Classes
* Functions

But also:

* Architectural components
* Modules
* Concepts
* Decision records
* Documentation units

Identity enables:

* Cross-reference resolution
* Offline indexing
* Stable linking
* Structured queries

Without identity, documentation is just prose.

---

## 7. Indexing as Infrastructure

Indexing is not optional.

Every serious codebase already relies on:

* IDE symbol resolution
* Find references
* Go to definition

This system generalizes that capability into a universal documentation substrate.

Indexing becomes:

* Language-aware
* Persistent
* Queryable outside the editor
* Exportable

---

## 8. Documentation as a Parallel Graph

Code forms one graph.

Documentation forms another.

The system must link them via:

* UID references
* Structural anchors
* Relevance relationships

This produces a dual-layer model:

```text
Code Graph ↔ Documentation Graph
```

Both evolving together.

---

## 9. Multi-Interface, Single Source

The core engine must power:

* CLI workflows
* MCP servers for AI
* VS Code extension
* JetBrains plugin
* CI exports
* Static renderers (optional)

There must be:

One index
One documentation store
One identity system

Multiple clients.

---

## 10. Token Efficiency as Engineering Discipline

The system must enforce:

* Field-selectable responses
* Bounded snippets
* Ranking by relevance
* No unnecessary payloads
* Structured JSON contracts

Precision over verbosity.

Signal over noise.

---

## 11. Beyond Documentation

In the long term, this system enables:

* Impact analysis before refactor
* Architecture visualization
* Dead code detection
* Inconsistency detection
* Cross-team knowledge discovery
* AI-assisted change planning

It becomes:

A semantic mirror of the codebase.

---

## 12. The Ideal End State

A developer working on a function should be able to ask:

* What is this?
* Why does it exist?
* Where is it used?
* What depends on it?
* What architectural decision is tied to it?

And receive:

* Ranked references
* Anchored documentation
* Contextually bounded explanations
* Structured machine-readable output

Without loading the entire repository.

Without guesswork.

Without noise.

---

## 13. Guiding Beliefs

We believe:

* Structure beats prose.
* Identity beats text matching.
* Anchors must survive change.
* AI must operate on structured substrates.
* Documentation must be durable.
* Indexing must be persistent.
* Rendering is optional.
* Querying is mandatory.

---

This is not a documentation tool.

This is a documentation infrastructure.

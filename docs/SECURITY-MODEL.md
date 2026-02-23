# Security Model

---

## 1. Purpose

This document defines the security model for the system.

The system:

* Parses source code
* Stores structured index data
* Stores documentation sidecar files
* Exposes MCP interface
* Integrates with IDE plugins
* Provides CLI access

Security must protect against:

* Code execution
* Injection attacks
* Data corruption
* Index poisoning
* Path traversal
* Denial-of-service
* Malformed input exploitation
* Unsafe LLM interaction patterns

Security is mandatory.

---

## 2. Threat Model

The system operates:

* On developer machines
* In CI environments
* Possibly inside containers

Threat actors may:

* Commit malicious code to repository
* Commit malicious sidecar files
* Provide malformed MCP inputs
* Attempt UID injection
* Attempt path traversal
* Attempt oversized query abuse
* Attempt to crash indexer
* Attempt to exploit IDE plugin

The system must assume:

Source code and sidecar files are untrusted input.

---

## 3. Trust Boundaries

### Trusted:

* Core binary code
* MCP server implementation
* CLI implementation
* IDE plugin code

### Untrusted:

* Source files
* Sidecar documentation files
* LSIF / SCIP import files
* MCP client inputs
* CLI arguments
* YAML metadata
* User-provided search queries

All untrusted inputs must be validated.

---

## 4. Code Execution Prohibition

The system must:

* Never execute source code
* Never evaluate macros dynamically
* Never invoke project build scripts
* Never run package manager hooks
* Never evaluate embedded YAML expressions
* Never evaluate documentation markdown as code

Tree-sitter parsing is safe because it parses text only.

No eval-like behavior allowed.

---

## 5. Path Traversal Protection

Sidecar file resolution must:

* Normalize paths
* Reject `../`
* Reject absolute path escapes
* Restrict to project root

Example:

Invalid:

```text
../../../etc/passwd
```

Must reject.

All file system access must be project-scoped.

---

## 6. UID Validation

All UID inputs must:

* Match strict regex
* Restrict allowed characters
* Reject control characters
* Reject path injection
* Reject extremely long strings

Example allowed pattern:

```text
^([a-z]+:)[a-z0-9._:/-]+$
```

Reject malformed UID before query execution.

---

## 7. YAML Parsing Safety

Sidecar metadata parser must:

* Use safe YAML parser mode
* Disable arbitrary object deserialization
* Reject custom tags
* Reject anchors & aliases if unsafe
* Limit file size
* Validate schema strictly

No dynamic object construction.

---

## 8. LSIF / SCIP Import Safety

Importer must:

* Treat LSIF/SCIP as untrusted
* Validate schema
* Validate symbol IDs
* Validate ranges
* Reject extremely large files
* Limit memory usage
* Avoid dynamic evaluation

Never trust external symbol IDs blindly.

---

## 9. MCP Input Validation

All MCP tool inputs must:

* Be validated against JSON schema
* Reject unknown fields
* Enforce max lengths
* Enforce max limits
* Reject injection attempts
* Reject complex nested queries

Query complexity must be bounded.

---

## 10. Denial-of-Service Protection

System must enforce:

* Maximum references returned
* Maximum recursion depth
* Maximum query size
* Maximum documentation size
* Timeout per request
* Rate limit per client (optional)

Prevent:

* Infinite graph traversal
* Memory exhaustion
* Large response payloads

---

## 11. Database Safety

Storage layer must:

* Use parameterized queries
* Prevent SQL injection
* Validate inputs before query
* Enforce transaction boundaries
* Avoid arbitrary SQL execution

No raw string concatenation allowed.

---

## 12. Index Corruption Prevention

System must:

* Use atomic writes
* Use transaction commits
* Use index version validation
* Detect incomplete rebuild
* Detect schema mismatch
* Support rebuild from source

Never leave partially corrupted index.

---

## 13. IDE Plugin Security

Plugin must:

* Only connect to local MCP by default
* Validate response schema
* Reject malformed JSON
* Not execute returned markdown
* Not evaluate arbitrary scripts
* Not allow remote MCP unless explicitly configured
* Not open arbitrary file paths

Plugin must respect JetBrains and VS Code security sandbox.

---

## 14. CLI Safety

CLI must:

* Validate UID input
* Validate file paths
* Prevent destructive commands without `--force`
* Not execute arbitrary shell commands
* Not expose internal file paths in errors
* Exit safely on malformed input

CLI must not auto-evaluate documentation content.

---

## 15. Logging Safety

Logs must:

* Avoid storing full documentation content
* Avoid storing full source code
* Avoid storing secrets
* Avoid exposing project absolute paths
* Redact sensitive paths if necessary

Logs must be minimal and structured.

---

## 16. AI Interaction Safety

When interacting with AI:

System must:

* Avoid injecting raw file content unnecessarily
* Avoid exposing secrets from codebase automatically
* Avoid automatically sending entire project graph
* Require explicit user request for large expansions

Token economy strategy reduces attack surface.

---

## 17. Generated Code Handling

Generated files may contain:

* Large blobs
* Embedded malicious patterns
* Excessively deep AST

System must allow:

* Exclusion patterns
* Generated file flag
* Size-based ignore rule

Avoid parsing extremely large files.

---

## 18. Resource Limits

System must enforce:

* Max file size for parsing
* Max documentation file size
* Max LSIF file size
* Max query time
* Max memory usage (configurable)

Prevent resource exhaustion.

---

## 19. Upgrade and Migration Safety

When schema changes:

* Validate migration scripts
* Backup existing index
* Fail safely if migration fails
* Allow rebuild fallback

No silent destructive migration.

---

## 20. Secure Defaults

Default configuration must:

* Disable remote MCP
* Disable embedding-based features
* Enable strict UID validation
* Enable input size limits
* Enable anchor validation warnings

Secure by default.

---

## 21. Non-Goals

Security model does not:

* Provide sandbox for arbitrary code execution
* Replace repository access control
* Replace OS-level security
* Provide network isolation
* Replace CI secrets management

System is local tooling.

---

## 22. Security Testing

Must include:

* Malformed UID tests
* Path traversal tests
* YAML injection tests
* Oversized file tests
* Query abuse tests
* LSIF corruption tests
* MCP injection tests
* SQL injection tests
* Plugin malformed response tests

Security regressions must fail CI.

---

## 23. Incident Response

If vulnerability found:

* Patch quickly
* Bump schema version if needed
* Provide migration instructions
* Notify users in changelog
* Add regression test

Security must evolve.

---

## 24. Summary

Security model ensures:

* No code execution
* No injection vulnerabilities
* No path traversal
* No index poisoning
* No silent corruption
* No unbounded resource abuse
* Safe IDE integration
* Safe MCP exposure

All external input is untrusted.

Validation is mandatory.

Determinism reduces attack surface.

Security is part of architecture, not an afterthought.
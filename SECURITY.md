Security Policy

Supported Versions

The project is currently in active development.

Security fixes will be applied to the latest main branch.

Pre-1.0 releases do not guarantee backward compatibility, but critical vulnerabilities will be addressed promptly.

⸻

Reporting a Vulnerability

If you discover a security vulnerability, please do not open a public issue.

Instead:
	•	Email: sidecar@dsxm.org
	•	Subject line: Security Vulnerability Report

Include:
	•	Description of the issue
	•	Steps to reproduce
	•	Affected version / commit hash
	•	Potential impact assessment (if known)
	•	Suggested mitigation (optional)

You will receive an acknowledgment within 72 hours.

⸻

Disclosure Policy
	•	Vulnerabilities will be triaged privately.
	•	A fix will be prepared and reviewed.
	•	A security advisory will be published after patch release.
	•	Credit will be given to reporters unless anonymity is requested.

We follow a responsible disclosure process.

⸻

Scope

Security considerations include:
	•	UID validation
	•	Path traversal protection
	•	YAML parsing safety
	•	MCP input validation
	•	Resource exhaustion protection
	•	Deterministic query guarantees
	•	Sidecar file integrity

Out-of-scope issues include:
	•	Misuse of the software
	•	Unsupported environments
	•	Issues in third-party dependencies (report to upstream)

⸻

Security Guarantees

The system is designed to:
	•	Never execute source code
	•	Never evaluate documentation content
	•	Treat all repository content as untrusted input
	•	Validate all external inputs
	•	Enforce bounded queries
	•	Prevent unbounded graph expansion
	•	Protect against path traversal
	•	Reject malformed UID and metadata
	•	Maintain deterministic behavior

See docs/SECURITY-MODEL.md for full architectural details.

⸻

Security Updates

Security updates will be documented in:
	•	CHANGELOG.md
	•	GitHub Security Advisories (when applicable)

⸻

Security is part of the architecture, not an afterthought.
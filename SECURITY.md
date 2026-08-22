# Security Policy

## Reporting a Vulnerability

We take security issues in Modern Production Research (MPR) seriously.

If you discover a security vulnerability, please do not open a public GitHub issue.

Instead, report the issue privately through GitHub's security reporting mechanism for this repository.

Please include:

- A clear description of the vulnerability
- The affected component or file
- Steps to reproduce the issue
- The potential impact
- Any relevant logs, traces, or proof-of-concept details
- A suggested mitigation, if available

## Scope

Security reports may include issues involving:

- Authentication or authorization
- Unsafe request routing
- Resource exhaustion
- Rate-limiting bypasses
- Dependency isolation
- Memory-safety issues in Rust or eBPF components
- Information disclosure
- Remote code execution
- Container or infrastructure configuration

## Responsible Disclosure

Please allow maintainers reasonable time to investigate and address a reported vulnerability before publicly disclosing technical details.

We appreciate responsible security research and will acknowledge valid reports when appropriate.

## Security Updates

Security fixes will be documented through the repository's release notes, security advisories, or relevant changelog entries when applicable.

## Out of Scope

Reports that do not demonstrate a reproducible security impact may not qualify as security vulnerabilities.

For general bugs, feature requests, or performance issues, please use the repository's standard issue tracker.
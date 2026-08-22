# MPR Maintainers & Architecture Registry

This document describes project ownership, review responsibilities, and escalation paths.

## 1. Core Maintainer

- **Devesh Chauhan (@devloperdevesh)** — Project maintainer and architecture owner.

Areas of responsibility include distributed systems architecture, asynchronous execution, resilience primitives, and infrastructure design.

## 2. Code Review & Merging

Pull requests should:

1. Pass required automated validation checks.
2. Include appropriate tests for changed behavior.
3. Include benchmark evidence for performance-sensitive changes.
4. Avoid introducing unnecessary blocking operations into asynchronous paths.
5. Document significant architectural tradeoffs.

## 3. Escalation

For architectural questions or disagreements, open an Issue with enough technical context for maintainers to reproduce and evaluate the concern.

Use the appropriate repository component labels where available.
# Modern Production Research (MPR) Architectural Contributing Guidelines

This document establishes mandatory architectural constraints, runtime invariants, and validation protocols for modifications to the MPR control plane.

## 1. Architectural Core Invariants

All code paths should preserve the following engineering principles.

### A. Non-Blocking Asynchronous Execution

Network transactions, rate-control interceptors, and circuit-breaker state machines should use non-blocking asynchronous execution.

- Avoid thread-blocking primitives such as `time.sleep()` in asynchronous paths.
- Prefer `asyncio.sleep()` for asynchronous delays.
- Prefer asynchronous connectors such as `redis.asyncio` and `asyncpg` where applicable.

### B. Static Type & Linting

Python modules should use appropriate type hints.

Code changes should pass the repository's configured linting and validation checks.

### C. Empirical Performance Benchmarking

Performance-sensitive modifications should include:

- Local simulation using `/experiments`.
- Relevant telemetry measurements.
- Tail-latency measurements such as P50, P99, and P999 where applicable.

## 2. Pull Request & Triage

1. Create an isolated feature branch from `main`.
2. Provision the local infrastructure using:

   `docker compose -f infra/docker-compose.yml up -d`

3. Run the local test suite.
4. Document relevant architectural tradeoffs in `/docs/research`.
5. Include reproducible benchmark information for performance-sensitive changes.
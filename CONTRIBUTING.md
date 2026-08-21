# Modern Production Research (MPR) Contributing Guidelines

We welcome backend engineers, distributed systems researchers, and systems performance specialists. To maintain enterprise-grade reliability and latency invariants across the MPR runtime, all contributions must strictly conform to these structural guardrails.

## Core Engineering Invariants

### 1. Absolute Asynchronous Runtime Guard
Every transaction orchestration execution route, rate-control hook, or circuit-tripping state machine within the `/core` boundary must use native asynchronous loops (`async`/`await`).
* Thread-blocking OS commands (such as Python's native `time.sleep()`) are strictly forbidden. Use `asyncio.sleep()`.
* Network drivers must remain fully non-blocking. Synchronous database or cache connectors are rejected; utilize `redis.asyncio` and `asyncpg` exclusively.

### 2. Cache-Line Memory Alignment (Rust Data Path)
Data primitives handling high-concurrency cross-cluster locking or localized queue states within `/src` must be structured with explicit consideration for hardware memory cache-lines. Use explicit `#[repr(align(64))]` or cache-line structural padding to prevent false sharing and atomic bus lock degradation across concurrent CPU cores.

### 3. Empirical Telemetry Benchmarking Requirement
If your Pull Request implements an optimization or structural mitigation pattern, you are strictly required to supply performance evidence:
* Execute a simulated load loop utilizing the tools located in `/experiments` under a continuous load profile using our custom Locust setup.
* Append the analytical telemetry output (specifically $P50$, $P99$, and $P999$ tail-latency behavior) directly to your Pull Request description. Any degradation to nominal data paths will be automatically flagged by our integration checks.

## Pull Request Lifecycle

1. Fork the master branch and initialize your custom feature track.
2. Initialize local node clusters via `docker-compose -f infra/docker-compose.yml up -d`.
3. Run strict static type evaluations and code linting audits (`ruff check`) via the configurations defined in our CI pipeline.
4. Document the precise architectural tradeoffs or compute compromises introduced by your structural adjustments inside the corresponding `/docs/research` logs.

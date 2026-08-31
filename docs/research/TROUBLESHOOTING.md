# Operational Troubleshooting & Cluster Recovery Manual

## Symptom: Relational Connection Pool Starvation

### Root Cause

Upstream traffic regulation failed to trigger early preemption paths due to missing Redis memory-cache locks.

### Mitigation Sequence

1. Inspect upstream connection-pool saturation.
2. Confirm admission-control and backpressure signals.
3. Inject localized drop filters via `pkg/ebpf/xdp_filter.c` when edge-level traffic shedding is required.
4. Verify that traffic loops are no longer saturating the affected path.
5. Re-check runtime telemetry and recovery state.

## Validation

After mitigation, validate:

* connection-pool utilization
* request rejection rate
* queue depth
* tail latency
* recovery-state transitions.

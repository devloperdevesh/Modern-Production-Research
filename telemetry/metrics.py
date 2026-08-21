import time
from typing import Callable
from fastapi import Request, Response
from prometheus_client import Counter, Histogram, Gauge

SYSTEM_THROUGHPUT = Counter("mpr_throughput_total", "Total transaction velocity tracks", ["status"])
LATENCY_PROFILE = Histogram("mpr_latency_seconds", "High-res latency distribution trajectories", buckets=(0.005, 0.05, 0.5, 2.5))
CONCURRENT_JOBS = Gauge("mpr_concurrent_executions", "Active runtime thread tracking allocations")

async def observability_middleware(request: Request, call_next: Callable) -> Response:
    CONCURRENT_JOBS.inc()
    start_time = time.perf_counter()
    try:
        response = await call_next(request)
        SYSTEM_THROUGHPUT.labels(status="success" if response.status_code < 400 else "failure").inc()
        return response
    finally:
        LATENCY_PROFILE.observe(time.perf_counter() - start_time)
        CONCURRENT_JOBS.dec()

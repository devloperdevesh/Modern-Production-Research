import uuid
from typing import Dict, Any

class OpenTelemetryContextPropagator:
    """
    Low-overhead distributed context propagation interceptor for MPR.
    Injects and extracts trace state metrics across asynchronous microservice boundaries
    without blocking the core network application runtime loops.
    """
    def __init__(self):
        self.context_registry: Dict[str, Dict[str, Any]] = {}

    async def inject_context_headers(self, carrier_headers: Dict[str, str], tenant_id: str) -> str:
        """Injects trace identifiers into outbound metadata carrier maps."""
        trace_id = str(uuid.uuid4())
        carrier_headers["X-MPR-Trace-Id"] = trace_id
        carrier_headers["X-MPR-Tenant-Id"] = tenant_id
        
        self.context_registry[trace_id] = {
            "tenant_id": tenant_id,
            "timestamp": time.time()
        }
        return trace_id

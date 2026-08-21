import time
from typing import Dict, Any, Tuple

class DynamicConcurrencyLoadShedder:
    """
    Predictive application-layer admission regulator for MPR.
    Monitors rolling tail latency via Exponential Moving Average (EMA) to execute
    immediate load shedding before downstream thread starvation triggers a host crash.
    """
    def __init__(self, critical_latency_threshold_secs: float = 0.500, alpha: float = 0.2):
        self.threshold = critical_latency_threshold_secs
        self.alpha = alpha  
        self.rolling_latency_average: float = 0.005  

    async def evaluate_transaction(self, request_metadata: Dict[str, Any]) -> Tuple[bool, str]:
        """
        Evaluates system health boundaries. Rejects non-critical traffic during saturation cycles.
        """
        if self.rolling_latency_average > self.threshold:
            if request_metadata.get("priority") != "CRITICAL":
                return False, "REJECTED_SYSTEM_SATURATED"
        return True, "PERMITTED"

    async def register_execution_feedback(self, execution_duration: float):
        """
        Updates the mathematical mathematical moving average distribution trajectory without locks.
        """
        self.rolling_latency_average = (execution_duration * self.alpha) + (self.rolling_latency_average * (1 - self.alpha))

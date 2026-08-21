import asyncio
import logging

logger = logging.getLogger("mpr.experiments.chaos")

class NetworkPartitionSimulator:
    """Injects deterministic failures into localized cluster routing tables."""
    def __init__(self, target_node: str):
        self.target_node = target_node
        self.is_partitioned = False

    async def inject_fault(self, duration_secs: float):
        """Simulates immediate network drop on destination infrastructure pool."""
        logger.warning(f"MPR-Chaos: Dropping connection channels to {self.target_node}")
        self.is_partitioned = True
        await asyncio.sleep(duration_secs)
        logger.info(f"MPR-Chaos: Restoring cluster topologies for {self.target_node}")
        self.is_partitioned = False

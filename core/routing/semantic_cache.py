import time
import math
from typing import Dict, Optional, Tuple, Any

class AdvancedSemanticCache:
    """
    10x Corporate Revenue Moat for MPR.
    Intercepts repetitive high-volume AI context prompts and executes low-latency
    vector-space comparisons to stop redundant downstream token burn rates.
    """
    def __init__(self, threshold: float = 0.85):
        self.threshold = threshold  
        self.vector_store: Dict[str, Tuple[list, Any, float]] = {}

    def _calculate_cosine_similarity(self, vec1: list, vec2: list) -> float:
        """Computes dot-product similarity profile between two context vectors."""
        if len(vec1) != len(vec2): return 0.0
        dot_product = sum(a * b for a, b in zip(vec1, vec2))
        magnitude_v1 = math.sqrt(sum(a * a for a in vec1))
        magnitude_v2 = math.sqrt(sum(b * b for b in vec2))
        if magnitude_v1 == 0 or magnitude_v2 == 0: return 0.0
        return dot_product / (magnitude_v1 * magnitude_v2)

    async def lookup(self, prompt_embedding: list) -> Tuple[Optional[Any], float]:
        """Queries local high-throughput runtime memory layers for valid tokens."""
        best_match_payload = None
        highest_similarity = -1.0
        for cache_key, (cached_embedding, payload, expires_at) in self.vector_store.items():
            if time.time() > expires_at: continue
            similarity = self._calculate_cosine_similarity(prompt_embedding, cached_embedding)
            if similarity > highest_similarity:
                highest_similarity = similarity
                best_match_payload = payload
        if highest_similarity >= self.threshold:
            return best_match_payload, highest_similarity
        return None, highest_similarity

    async def insert(self, key_identifier: str, prompt_embedding: list, response_payload: Any, ttl: float = 3600.0):
        """Commits processed state payloads dynamically to the memory segment."""
        self.vector_store[key_identifier] = (prompt_embedding, response_payload, time.time() + ttl)

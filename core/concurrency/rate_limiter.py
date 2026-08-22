import time
import redis.asyncio as aioredis


class RedisAtomicTokenBucket:
    """
    Distributed, cluster-aware Token Bucket rate limiter for MPR.
    Executes structural token allocation via Lua scripts inside Redis memory space
    to completely eliminate concurrency race conditions under distributed deployment clusters.
    """

    def __init__(self, redis_pool: aioredis.Redis):
        self.redis = redis_pool
        self.lua_script = """
            local key = KEYS[1]
            local capacity = tonumber(ARGV[1])
            local refill_rate = tonumber(ARGV[2])
            local requested = tonumber(ARGV[3])
            local now = tonumber(ARGV[4])

            local state = redis.call('HMGET', key, 'tokens', 'last_updated')
            local tokens = tonumber(state[1])
            local last_updated = tonumber(state[2])

            if tokens == nil then
                tokens = capacity
                last_updated = now
            else
                local elapsed = now - last_updated
                if elapsed > 0 then
                    tokens = math.min(capacity, tokens + (elapsed * refill_rate))
                end
            end

            if tokens >= requested then
                tokens = tokens - requested
                redis.call('HMSET', key, 'tokens', tokens, 'last_updated', now)
                redis.call('EXPIRE', key, 86400)
                return 1
            else
                redis.call('HMSET', key, 'tokens', tokens, 'last_updated', now)
                return 0
            end
        """
        self.script_executor = self.redis.register_script(self.lua_script)

    async def evaluate(
        self, identifier: str, capacity: int, refill_rate: float
    ) -> bool:
        """
        Atomically evaluates systemic boundaries for incoming execution streams.
        Returns True if capacity exists, False if threshold breached (Shed Traffic).
        """
        key = f"mpr:limiter:{identifier}"
        now = time.time()
        try:
            result = await self.script_executor(
                keys=[key], args=[capacity, refill_rate, 1, now]
            )
            return bool(result)
        except Exception:
            # Production resilience principle: Fail-Open to secure application uptime
            return True

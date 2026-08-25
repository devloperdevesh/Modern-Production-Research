use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TokenBucket {
    capacity: u64,
    refill_rate: u64,
    tokens: AtomicU64,
    last_updated: AtomicU64,
}

impl TokenBucket {
    pub fn new(capacity: u64, refill_rate: u64) -> Self {
        let now = current_time_secs();

        Self {
            capacity,
            refill_rate,
            tokens: AtomicU64::new(capacity),
            last_updated: AtomicU64::new(now),
        }
    }

    pub fn acquire(&self, requested: u64) -> bool {
        if requested == 0 {
            return true;
        }

        if requested > self.capacity {
            return false;
        }

        self.refill();

        loop {
            let current = self.tokens.load(Ordering::Acquire);

            if current < requested {
                return false;
            }

            if self
                .tokens
                .compare_exchange_weak(
                    current,
                    current - requested,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
                .is_ok()
            {
                return true;
            }
        }
    }

    pub fn available(&self) -> u64 {
        self.refill();
        self.tokens.load(Ordering::Acquire)
    }

    fn refill(&self) {
        let now = current_time_secs();
        let last = self.last_updated.load(Ordering::Acquire);
        let elapsed = now.saturating_sub(last);

        if elapsed == 0 {
            return;
        }

        let current = self.tokens.load(Ordering::Acquire);
        let refill_amount = elapsed.saturating_mul(self.refill_rate);
        let new_tokens = current.saturating_add(refill_amount).min(self.capacity);

        self.tokens.store(new_tokens, Ordering::Release);
        self.last_updated.store(now, Ordering::Release);
    }
}

fn current_time_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_requests_within_capacity() {
        let bucket = TokenBucket::new(10, 1);

        assert!(bucket.acquire(5));
        assert!(bucket.acquire(5));
        assert!(!bucket.acquire(1));
    }

    #[test]
    fn rejects_request_larger_than_capacity() {
        let bucket = TokenBucket::new(10, 1);

        assert!(!bucket.acquire(11));
    }

    #[test]
    fn zero_request_does_not_consume_tokens() {
        let bucket = TokenBucket::new(10, 1);

        assert!(bucket.acquire(0));
        assert_eq!(bucket.available(), 10);
    }

    #[test]
    fn zero_capacity_rejects_positive_request() {
        let bucket = TokenBucket::new(0, 1);

        assert!(!bucket.acquire(1));
    }
}

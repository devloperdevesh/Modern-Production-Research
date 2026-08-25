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
        Self {
            capacity,
            refill_rate,
            tokens: AtomicU64::new(capacity),
            last_updated: AtomicU64::new(Self::now()),
        }
    }

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    pub fn try_acquire(&self, requested: u64) -> bool {
        if requested == 0 || requested > self.capacity {
            return false;
        }

        let now = Self::now();
        let last = self.last_updated.load(Ordering::Acquire);
        let elapsed = now.saturating_sub(last);

        if elapsed > 0 {
            let current = self.tokens.load(Ordering::Acquire);
            let refill = elapsed.saturating_mul(self.refill_rate);
            let updated = current.saturating_add(refill).min(self.capacity);

            let _ =
                self.tokens
                    .compare_exchange(current, updated, Ordering::AcqRel, Ordering::Acquire);

            let _ =
                self.last_updated
                    .compare_exchange(last, now, Ordering::AcqRel, Ordering::Acquire);
        }

        loop {
            let current = self.tokens.load(Ordering::Acquire);

            if current < requested {
                return false;
            }

            match self.tokens.compare_exchange(
                current,
                current - requested,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return true,
                Err(_) => continue,
            }
        }
    }

    pub fn available(&self) -> u64 {
        self.tokens.load(Ordering::Acquire)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_requests_within_capacity() {
        let bucket = TokenBucket::new(10, 1);

        assert!(bucket.try_acquire(5));
        assert!(bucket.try_acquire(5));
        assert!(!bucket.try_acquire(1));
    }

    #[test]
    fn rejects_invalid_requests() {
        let bucket = TokenBucket::new(10, 1);

        assert!(!bucket.try_acquire(0));
        assert!(!bucket.try_acquire(11));
    }
}

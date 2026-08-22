use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct BareMetalTokenBucket {
    capacity: u64,
    refill_rate: u64,
    tokens: AtomicU64,
    last_updated: AtomicU64,
}

impl BareMetalTokenBucket {
    pub fn new(capacity: u64, refill_rate: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self {
            capacity,
            refill_rate,
            tokens: AtomicU64::new(capacity),
            last_updated: AtomicU64::new(now),
        }
    }

    pub fn acquire(&self, requested: u64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let last_time = self.last_updated.load(Ordering::Relaxed);
        let elapsed = now.saturating_sub(last_time);

        if elapsed > 0 {
            let current_tokens = self.tokens.load(Ordering::Relaxed);
            let refilled =
                std::cmp::min(self.capacity, current_tokens + (elapsed * self.refill_rate));
            self.tokens.store(refilled, Ordering::Relaxed);
            self.last_updated.store(now, Ordering::Relaxed);
        }

        loop {
            let current = self.tokens.load(Ordering::SeqCst);
            if current < requested {
                return false;
            }
            if self
                .tokens
                .compare_exchange(
                    current,
                    current - requested,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                )
                .is_ok()
            {
                return true;
            }
        }
    }
}

fn main() {
    println!("MPR Bare-Metal Engine: Active and bounded to native system clock loops.");
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_bucket_allows_requests_within_capacity() {
        let bucket = BareMetalTokenBucket::new(10, 1);

        assert!(bucket.acquire(5));
        assert!(bucket.acquire(5));
    }

    #[test]
    fn token_bucket_rejects_requests_above_available_tokens() {
        let bucket = BareMetalTokenBucket::new(10, 1);

        assert!(bucket.acquire(10));
        assert!(!bucket.acquire(1));
    }

    #[test]
    fn token_bucket_rejects_zero_capacity_requests() {
        let bucket = BareMetalTokenBucket::new(0, 1);

        assert!(!bucket.acquire(1));
    }
}

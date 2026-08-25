use std::sync::atomic::{AtomicU64, Ordering};

pub struct ConcurrencyLimiter {
    limit: u64,
    active: AtomicU64,
}

impl ConcurrencyLimiter {
    pub const fn new(limit: u64) -> Self {
        Self {
            limit,
            active: AtomicU64::new(0),
        }
    }

    pub fn try_acquire(&self) -> bool {
        loop {
            let current = self.active.load(Ordering::Acquire);

            if current >= self.limit {
                return false;
            }

            if self
                .active
                .compare_exchange(current, current + 1, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return true;
            }
        }
    }

    pub fn release(&self) {
        self.active
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
                current.checked_sub(1)
            })
            .ok();
    }

    pub fn active(&self) -> u64 {
        self.active.load(Ordering::Acquire)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enforces_concurrency_limit() {
        let limiter = ConcurrencyLimiter::new(2);

        assert!(limiter.try_acquire());
        assert!(limiter.try_acquire());
        assert!(!limiter.try_acquire());

        limiter.release();

        assert!(limiter.try_acquire());
    }
}

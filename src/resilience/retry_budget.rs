use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryDecision {
    Allow,
    Exhausted,
}

pub struct RetryBudget {
    remaining: AtomicU64,
    limit: u64,
}

impl RetryBudget {
    pub const fn new(limit: u64) -> Self {
        Self {
            remaining: AtomicU64::new(limit),
            limit,
        }
    }

    pub const fn limit(&self) -> u64 {
        self.limit
    }

    pub fn remaining(&self) -> u64 {
        self.remaining.load(Ordering::Acquire)
    }

    pub fn try_acquire(&self) -> RetryDecision {
        loop {
            let current = self.remaining.load(Ordering::Acquire);

            if current == 0 {
                return RetryDecision::Exhausted;
            }

            if self
                .remaining
                .compare_exchange_weak(current, current - 1, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return RetryDecision::Allow;
            }
        }
    }

    pub fn reset(&self) {
        self.remaining.store(self.limit, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn allows_retries_within_budget() {
        let budget = RetryBudget::new(2);

        assert_eq!(budget.try_acquire(), RetryDecision::Allow);
        assert_eq!(budget.try_acquire(), RetryDecision::Allow);
        assert_eq!(budget.try_acquire(), RetryDecision::Exhausted);
        assert_eq!(budget.remaining(), 0);
    }

    #[test]
    fn reset_restores_budget() {
        let budget = RetryBudget::new(2);

        assert_eq!(budget.try_acquire(), RetryDecision::Allow);
        assert_eq!(budget.remaining(), 1);

        budget.reset();

        assert_eq!(budget.remaining(), 2);
        assert_eq!(budget.try_acquire(), RetryDecision::Allow);
    }

    #[test]
    fn zero_budget_is_exhausted() {
        let budget = RetryBudget::new(0);

        assert_eq!(budget.try_acquire(), RetryDecision::Exhausted);
    }

    #[test]
    fn supports_concurrent_budget_consumption() {
        let budget = Arc::new(RetryBudget::new(100));
        let mut handles = Vec::new();

        for _ in 0..8 {
            let budget = Arc::clone(&budget);

            handles.push(thread::spawn(move || {
                let mut allowed = 0;

                for _ in 0..20 {
                    if budget.try_acquire() == RetryDecision::Allow {
                        allowed += 1;
                    }
                }

                allowed
            }));
        }

        let total_allowed: usize = handles
            .into_iter()
            .map(|handle| handle.join().expect("retry worker panicked"))
            .sum();

        assert_eq!(total_allowed, 100);
        assert_eq!(budget.remaining(), 0);
    }
}

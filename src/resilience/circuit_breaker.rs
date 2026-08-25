use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CircuitState {
    Closed = 0,
    Open = 1,
    HalfOpen = 2,
}

impl CircuitState {
    fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::Open,
            2 => Self::HalfOpen,
            _ => Self::Closed,
        }
    }
}

pub struct CircuitBreaker {
    state: AtomicU8,
    failures: AtomicU64,
    failure_threshold: u64,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u64) -> Self {
        assert!(
            failure_threshold > 0,
            "failure threshold must be greater than zero"
        );

        Self {
            state: AtomicU8::new(CircuitState::Closed as u8),
            failures: AtomicU64::new(0),
            failure_threshold,
        }
    }

    pub fn state(&self) -> CircuitState {
        CircuitState::from_u8(self.state.load(Ordering::Acquire))
    }

    pub fn failure_count(&self) -> u64 {
        self.failures.load(Ordering::Acquire)
    }

    pub const fn failure_threshold(&self) -> u64 {
        self.failure_threshold
    }

    pub fn allow_request(&self) -> bool {
        match self.state() {
            CircuitState::Closed | CircuitState::HalfOpen => true,
            CircuitState::Open => false,
        }
    }

    pub fn record_success(&self) {
        self.failures.store(0, Ordering::Release);
        self.state
            .store(CircuitState::Closed as u8, Ordering::Release);
    }

    pub fn record_failure(&self) {
        let failures = self.failures.fetch_add(1, Ordering::AcqRel) + 1;

        if failures >= self.failure_threshold {
            self.state
                .store(CircuitState::Open as u8, Ordering::Release);
        }
    }

    pub fn transition_to_half_open(&self) -> bool {
        self.state
            .compare_exchange(
                CircuitState::Open as u8,
                CircuitState::HalfOpen as u8,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
    }

    pub fn reset(&self) {
        self.failures.store(0, Ordering::Release);
        self.state
            .store(CircuitState::Closed as u8, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_closed() {
        let breaker = CircuitBreaker::new(3);

        assert_eq!(breaker.state(), CircuitState::Closed);
        assert!(breaker.allow_request());
        assert_eq!(breaker.failure_count(), 0);
    }

    #[test]
    fn opens_after_failure_threshold() {
        let breaker = CircuitBreaker::new(3);

        breaker.record_failure();
        assert_eq!(breaker.state(), CircuitState::Closed);

        breaker.record_failure();
        assert_eq!(breaker.state(), CircuitState::Closed);

        breaker.record_failure();

        assert_eq!(breaker.state(), CircuitState::Open);
        assert!(!breaker.allow_request());
        assert_eq!(breaker.failure_count(), 3);
    }

    #[test]
    fn open_circuit_can_transition_to_half_open() {
        let breaker = CircuitBreaker::new(1);

        breaker.record_failure();

        assert_eq!(breaker.state(), CircuitState::Open);
        assert!(breaker.transition_to_half_open());
        assert_eq!(breaker.state(), CircuitState::HalfOpen);
        assert!(breaker.allow_request());
    }

    #[test]
    fn successful_probe_closes_circuit() {
        let breaker = CircuitBreaker::new(1);

        breaker.record_failure();
        assert!(breaker.transition_to_half_open());

        breaker.record_success();

        assert_eq!(breaker.state(), CircuitState::Closed);
        assert!(breaker.allow_request());
        assert_eq!(breaker.failure_count(), 0);
    }

    #[test]
    fn success_resets_failure_count() {
        let breaker = CircuitBreaker::new(3);

        breaker.record_failure();
        breaker.record_failure();

        assert_eq!(breaker.failure_count(), 2);

        breaker.record_success();

        assert_eq!(breaker.failure_count(), 0);
        assert_eq!(breaker.state(), CircuitState::Closed);
    }

    #[test]
    fn reset_restores_closed_state() {
        let breaker = CircuitBreaker::new(2);

        breaker.record_failure();
        breaker.record_failure();

        assert_eq!(breaker.state(), CircuitState::Open);

        breaker.reset();

        assert_eq!(breaker.state(), CircuitState::Closed);
        assert_eq!(breaker.failure_count(), 0);
        assert!(breaker.allow_request());
    }

    #[test]
    #[should_panic(expected = "failure threshold must be greater than zero")]
    fn rejects_zero_threshold() {
        let _ = CircuitBreaker::new(0);
    }
}

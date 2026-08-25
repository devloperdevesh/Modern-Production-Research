use super::circuit_breaker::{CircuitBreaker, CircuitState};
use super::recovery_state::{RecoveryState, RecoveryTracker};
use super::retry_budget::{RetryBudget, RetryDecision};
use super::timeout::{Timeout, TimeoutDecision};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResilienceDecision {
    Allow,
    RejectCircuitOpen,
    RetryExhausted,
    TimedOut,
}

pub struct ResiliencePolicy {
    circuit_breaker: CircuitBreaker,
    retry_budget: RetryBudget,
    timeout: Timeout,
    recovery: RecoveryTracker,
}

impl ResiliencePolicy {
    pub fn new(failure_threshold: u64, retry_limit: u64, timeout_millis: u64) -> Self {
        Self {
            circuit_breaker: CircuitBreaker::new(failure_threshold),
            retry_budget: RetryBudget::new(retry_limit),
            timeout: Timeout::from_millis(timeout_millis),
            recovery: RecoveryTracker::new(),
        }
    }

    pub fn allow_request(&self) -> ResilienceDecision {
        if !self.circuit_breaker.allow_request() {
            return ResilienceDecision::RejectCircuitOpen;
        }

        ResilienceDecision::Allow
    }

    pub fn record_failure(&self) {
        self.recovery.mark_degraded();
        self.circuit_breaker.record_failure();
    }

    pub fn record_success(&self) {
        self.circuit_breaker.record_success();
        self.recovery.mark_healthy();
    }

    pub fn try_retry(&self) -> ResilienceDecision {
        match self.retry_budget.try_acquire() {
            RetryDecision::Allow => ResilienceDecision::Allow,
            RetryDecision::Exhausted => ResilienceDecision::RetryExhausted,
        }
    }

    pub fn evaluate_elapsed(&self, elapsed: std::time::Duration) -> ResilienceDecision {
        match self.timeout.evaluate(elapsed) {
            TimeoutDecision::WithinDeadline => ResilienceDecision::Allow,
            TimeoutDecision::TimedOut => ResilienceDecision::TimedOut,
        }
    }

    pub fn transition_to_half_open(&self) -> bool {
        let transitioned = self.circuit_breaker.transition_to_half_open();

        if transitioned {
            self.recovery.begin_recovery();
        }

        transitioned
    }

    pub fn circuit_state(&self) -> CircuitState {
        self.circuit_breaker.state()
    }

    pub fn recovery_state(&self) -> RecoveryState {
        self.recovery.state()
    }

    pub fn retry_remaining(&self) -> u64 {
        self.retry_budget.remaining()
    }

    pub fn timeout(&self) -> Timeout {
        self.timeout
    }

    pub fn reset(&self) {
        self.circuit_breaker.reset();
        self.retry_budget.reset();
        self.recovery.mark_healthy();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_healthy_request() {
        let policy = ResiliencePolicy::new(3, 2, 100);

        assert_eq!(policy.allow_request(), ResilienceDecision::Allow);
    }

    #[test]
    fn rejects_request_when_circuit_opens() {
        let policy = ResiliencePolicy::new(2, 2, 100);

        policy.record_failure();
        policy.record_failure();

        assert_eq!(policy.circuit_state(), CircuitState::Open);
        assert_eq!(
            policy.allow_request(),
            ResilienceDecision::RejectCircuitOpen
        );
    }

    #[test]
    fn enforces_retry_budget() {
        let policy = ResiliencePolicy::new(3, 2, 100);

        assert_eq!(policy.try_retry(), ResilienceDecision::Allow);
        assert_eq!(policy.try_retry(), ResilienceDecision::Allow);
        assert_eq!(policy.try_retry(), ResilienceDecision::RetryExhausted);
    }

    #[test]
    fn enforces_timeout() {
        let policy = ResiliencePolicy::new(3, 2, 100);

        assert_eq!(
            policy.evaluate_elapsed(std::time::Duration::from_millis(50)),
            ResilienceDecision::Allow
        );

        assert_eq!(
            policy.evaluate_elapsed(std::time::Duration::from_millis(100)),
            ResilienceDecision::TimedOut
        );
    }

    #[test]
    fn recovery_tracks_failure_and_success() {
        let policy = ResiliencePolicy::new(3, 2, 100);

        policy.record_failure();

        assert_eq!(policy.recovery_state(), RecoveryState::Degraded);

        policy.record_success();

        assert_eq!(policy.recovery_state(), RecoveryState::Healthy);
    }

    #[test]
    fn reset_restores_policy_state() {
        let policy = ResiliencePolicy::new(2, 1, 100);

        policy.record_failure();
        policy.record_failure();
        let _ = policy.try_retry();

        policy.reset();

        assert_eq!(policy.circuit_state(), CircuitState::Closed);
        assert_eq!(policy.recovery_state(), RecoveryState::Healthy);
        assert_eq!(policy.retry_remaining(), 1);
    }
}

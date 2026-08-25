use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeoutDecision {
    WithinDeadline,
    TimedOut,
}

#[derive(Debug, Clone, Copy)]
pub struct Timeout {
    duration: Duration,
}

impl Timeout {
    pub const fn from_millis(milliseconds: u64) -> Self {
        Self {
            duration: Duration::from_millis(milliseconds),
        }
    }

    pub const fn duration(&self) -> Duration {
        self.duration
    }

    pub fn start(&self) -> TimeoutGuard {
        TimeoutGuard {
            deadline: Instant::now() + self.duration,
        }
    }

    pub fn evaluate(&self, elapsed: Duration) -> TimeoutDecision {
        if elapsed >= self.duration {
            TimeoutDecision::TimedOut
        } else {
            TimeoutDecision::WithinDeadline
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TimeoutGuard {
    deadline: Instant,
}

impl TimeoutGuard {
    pub fn check(&self) -> TimeoutDecision {
        if Instant::now() >= self.deadline {
            TimeoutDecision::TimedOut
        } else {
            TimeoutDecision::WithinDeadline
        }
    }

    pub fn remaining(&self) -> Duration {
        self.deadline.saturating_duration_since(Instant::now())
    }

    pub fn expired(&self) -> bool {
        matches!(self.check(), TimeoutDecision::TimedOut)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn accepts_work_within_deadline() {
        let timeout = Timeout::from_millis(100);

        assert_eq!(
            timeout.evaluate(Duration::from_millis(50)),
            TimeoutDecision::WithinDeadline
        );
    }

    #[test]
    fn rejects_work_at_deadline() {
        let timeout = Timeout::from_millis(100);

        assert_eq!(
            timeout.evaluate(Duration::from_millis(100)),
            TimeoutDecision::TimedOut
        );
    }

    #[test]
    fn guard_starts_with_remaining_time() {
        let timeout = Timeout::from_millis(100);
        let guard = timeout.start();

        assert!(!guard.expired());
        assert!(guard.remaining() <= Duration::from_millis(100));
    }

    #[test]
    fn guard_expires_after_deadline() {
        let timeout = Timeout::from_millis(1);
        let guard = timeout.start();

        thread::sleep(Duration::from_millis(5));

        assert!(guard.expired());
        assert_eq!(guard.check(), TimeoutDecision::TimedOut);
    }

    #[test]
    fn zero_timeout_expires_immediately() {
        let timeout = Timeout::from_millis(0);
        let guard = timeout.start();

        assert!(guard.expired());
        assert_eq!(guard.remaining(), Duration::ZERO);
    }
}

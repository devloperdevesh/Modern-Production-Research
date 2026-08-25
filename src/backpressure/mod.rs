#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backpressure {
    Open,
    Throttled,
}

pub struct QueueGuard {
    max_depth: usize,
}

impl QueueGuard {
    pub const fn new(max_depth: usize) -> Self {
        Self { max_depth }
    }

    pub const fn max_depth(&self) -> usize {
        self.max_depth
    }

    pub const fn evaluate(&self, current_depth: usize) -> Backpressure {
        if current_depth >= self.max_depth {
            Backpressure::Throttled
        } else {
            Backpressure::Open
        }
    }

    pub const fn is_throttled(&self, current_depth: usize) -> bool {
        matches!(self.evaluate(current_depth), Backpressure::Throttled)
    }

    pub const fn remaining_capacity(&self, current_depth: usize) -> usize {
        self.max_depth.saturating_sub(current_depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn throttles_full_queue() {
        let guard = QueueGuard::new(10);

        assert_eq!(guard.evaluate(10), Backpressure::Throttled);
        assert_eq!(guard.evaluate(9), Backpressure::Open);
    }

    #[test]
    fn throttles_when_queue_exceeds_limit() {
        let guard = QueueGuard::new(10);

        assert_eq!(guard.evaluate(11), Backpressure::Throttled);
        assert_eq!(guard.evaluate(100), Backpressure::Throttled);
    }

    #[test]
    fn reports_remaining_capacity() {
        let guard = QueueGuard::new(10);

        assert_eq!(guard.remaining_capacity(0), 10);
        assert_eq!(guard.remaining_capacity(6), 4);
        assert_eq!(guard.remaining_capacity(10), 0);
        assert_eq!(guard.remaining_capacity(20), 0);
    }

    #[test]
    fn exposes_throttled_state() {
        let guard = QueueGuard::new(4);

        assert!(!guard.is_throttled(3));
        assert!(guard.is_throttled(4));
        assert!(guard.is_throttled(5));
    }

    #[test]
    fn zero_capacity_always_throttles() {
        let guard = QueueGuard::new(0);

        assert_eq!(guard.evaluate(0), Backpressure::Throttled);
        assert_eq!(guard.evaluate(1), Backpressure::Throttled);
    }
}

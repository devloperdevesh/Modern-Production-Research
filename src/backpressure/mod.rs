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

    pub const fn evaluate(&self, current_depth: usize) -> Backpressure {
        if current_depth >= self.max_depth {
            Backpressure::Throttled
        } else {
            Backpressure::Open
        }
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
}

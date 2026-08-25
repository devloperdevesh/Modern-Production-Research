#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backpressure {
    Open,
    Throttled,
}

pub const fn evaluate(current_depth: usize, max_depth: usize) -> Backpressure {
    if current_depth >= max_depth {
        Backpressure::Throttled
    } else {
        Backpressure::Open
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn throttles_full_queue() {
        assert_eq!(evaluate(10, 10), Backpressure::Throttled);
    }
}

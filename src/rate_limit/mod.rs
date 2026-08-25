#[derive(Debug, Clone, Copy)]
pub struct RateLimit {
    capacity: u64,
    refill_rate: u64,
}

impl RateLimit {
    pub const fn new(capacity: u64, refill_rate: u64) -> Self {
        Self {
            capacity,
            refill_rate,
        }
    }

    pub const fn capacity(&self) -> u64 {
        self.capacity
    }

    pub const fn refill_rate(&self) -> u64 {
        self.refill_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_rate_limit_configuration() {
        let limiter = RateLimit::new(100, 10);
        assert_eq!(limiter.capacity(), 100);
        assert_eq!(limiter.refill_rate(), 10);
    }
}

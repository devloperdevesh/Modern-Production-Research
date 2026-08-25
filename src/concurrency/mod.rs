pub struct ConcurrencyController {
    limit: usize,
}

impl ConcurrencyController {
    pub const fn new(limit: usize) -> Self {
        Self { limit }
    }

    pub const fn limit(&self) -> usize {
        self.limit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_controller_with_limit() {
        let controller = ConcurrencyController::new(64);
        assert_eq!(controller.limit(), 64);
    }
}

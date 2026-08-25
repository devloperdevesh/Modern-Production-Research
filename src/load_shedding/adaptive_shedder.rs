use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShedDecision {
    Accept,
    Reject,
}

pub struct AdaptiveLoadShedder {
    threshold: u64,
    observed_load: AtomicU64,
}

impl AdaptiveLoadShedder {
    pub const fn new(threshold: u64) -> Self {
        Self {
            threshold,
            observed_load: AtomicU64::new(0),
        }
    }

    pub const fn threshold(&self) -> u64 {
        self.threshold
    }

    pub fn update_load(&self, current_load: u64) {
        self.observed_load.store(current_load, Ordering::Release);
    }

    pub fn current_load(&self) -> u64 {
        self.observed_load.load(Ordering::Acquire)
    }

    pub fn evaluate(&self) -> ShedDecision {
        if self.current_load() >= self.threshold {
            ShedDecision::Reject
        } else {
            ShedDecision::Accept
        }
    }

    pub fn evaluate_load(&self, current_load: u64) -> ShedDecision {
        if current_load >= self.threshold {
            ShedDecision::Reject
        } else {
            ShedDecision::Accept
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_load_below_threshold() {
        let shedder = AdaptiveLoadShedder::new(100);

        assert_eq!(shedder.evaluate_load(50), ShedDecision::Accept);
    }

    #[test]
    fn rejects_load_at_threshold() {
        let shedder = AdaptiveLoadShedder::new(100);

        assert_eq!(shedder.evaluate_load(100), ShedDecision::Reject);
    }

    #[test]
    fn rejects_load_above_threshold() {
        let shedder = AdaptiveLoadShedder::new(100);

        assert_eq!(shedder.evaluate_load(150), ShedDecision::Reject);
    }

    #[test]
    fn tracks_observed_load() {
        let shedder = AdaptiveLoadShedder::new(100);

        shedder.update_load(75);

        assert_eq!(shedder.current_load(), 75);
        assert_eq!(shedder.evaluate(), ShedDecision::Accept);
    }

    #[test]
    fn rejects_tracked_saturated_load() {
        let shedder = AdaptiveLoadShedder::new(100);

        shedder.update_load(100);

        assert_eq!(shedder.evaluate(), ShedDecision::Reject);
    }
}

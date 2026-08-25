use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RecoveryState {
    Healthy = 0,
    Degraded = 1,
    Recovering = 2,
    Failed = 3,
}

impl RecoveryState {
    fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::Degraded,
            2 => Self::Recovering,
            3 => Self::Failed,
            _ => Self::Healthy,
        }
    }
}

pub struct RecoveryTracker {
    state: AtomicU8,
}

impl RecoveryTracker {
    pub const fn new() -> Self {
        Self {
            state: AtomicU8::new(RecoveryState::Healthy as u8),
        }
    }

    pub fn state(&self) -> RecoveryState {
        RecoveryState::from_u8(self.state.load(Ordering::Acquire))
    }

    pub fn mark_degraded(&self) {
        self.state
            .store(RecoveryState::Degraded as u8, Ordering::Release);
    }

    pub fn begin_recovery(&self) -> bool {
        self.state
            .compare_exchange(
                RecoveryState::Degraded as u8,
                RecoveryState::Recovering as u8,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
    }

    pub fn mark_failed(&self) {
        self.state
            .store(RecoveryState::Failed as u8, Ordering::Release);
    }

    pub fn mark_healthy(&self) {
        self.state
            .store(RecoveryState::Healthy as u8, Ordering::Release);
    }

    pub fn is_healthy(&self) -> bool {
        self.state() == RecoveryState::Healthy
    }
}

impl Default for RecoveryTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_healthy() {
        let tracker = RecoveryTracker::new();

        assert_eq!(tracker.state(), RecoveryState::Healthy);
        assert!(tracker.is_healthy());
    }

    #[test]
    fn transitions_from_healthy_to_degraded() {
        let tracker = RecoveryTracker::new();

        tracker.mark_degraded();

        assert_eq!(tracker.state(), RecoveryState::Degraded);
        assert!(!tracker.is_healthy());
    }

    #[test]
    fn degraded_state_can_begin_recovery() {
        let tracker = RecoveryTracker::new();

        tracker.mark_degraded();

        assert!(tracker.begin_recovery());
        assert_eq!(tracker.state(), RecoveryState::Recovering);

        assert!(!tracker.begin_recovery());
    }

    #[test]
    fn recovery_can_restore_health() {
        let tracker = RecoveryTracker::new();

        tracker.mark_degraded();
        assert!(tracker.begin_recovery());

        tracker.mark_healthy();

        assert_eq!(tracker.state(), RecoveryState::Healthy);
        assert!(tracker.is_healthy());
    }

    #[test]
    fn failed_state_can_be_recovered() {
        let tracker = RecoveryTracker::new();

        tracker.mark_failed();
        assert_eq!(tracker.state(), RecoveryState::Failed);

        tracker.mark_healthy();

        assert_eq!(tracker.state(), RecoveryState::Healthy);
    }
}

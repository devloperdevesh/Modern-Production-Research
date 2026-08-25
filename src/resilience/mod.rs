#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthState {
    Healthy,
    Degraded,
    Saturated,
}

pub const fn is_available(state: HealthState) -> bool {
    !matches!(state, HealthState::Saturated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saturated_state_is_unavailable() {
        assert!(!is_available(HealthState::Saturated));
    }
}

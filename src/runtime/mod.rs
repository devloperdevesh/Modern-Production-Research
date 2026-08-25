#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeState {
    Starting,
    Running,
    Stopped,
}

pub const fn is_running(state: RuntimeState) -> bool {
    matches!(state, RuntimeState::Running)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn running_state_is_active() {
        assert!(is_running(RuntimeState::Running));
    }
}

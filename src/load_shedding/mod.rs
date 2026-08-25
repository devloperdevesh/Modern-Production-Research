#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Admission {
    Accept,
    Reject,
}

pub const fn evaluate(current_load: u64, limit: u64) -> Admission {
    if current_load >= limit {
        Admission::Reject
    } else {
        Admission::Accept
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_saturated_load() {
        assert_eq!(evaluate(100, 100), Admission::Reject);
    }
}

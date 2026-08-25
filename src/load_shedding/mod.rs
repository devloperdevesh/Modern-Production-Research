#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Admission {
    Accept,
    Reject,
}

pub struct LoadShedder {
    threshold: u64,
}

impl LoadShedder {
    pub const fn new(threshold: u64) -> Self {
        Self { threshold }
    }

    pub const fn evaluate(&self, current_load: u64) -> Admission {
        if current_load >= self.threshold {
            Admission::Reject
        } else {
            Admission::Accept
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_saturated_load() {
        let shedder = LoadShedder::new(100);

        assert_eq!(shedder.evaluate(100), Admission::Reject);
        assert_eq!(shedder.evaluate(50), Admission::Accept);
    }
}

pub mod adaptive_shedder;

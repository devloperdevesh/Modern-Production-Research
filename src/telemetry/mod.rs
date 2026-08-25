#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LatencySample {
    pub milliseconds: f64,
}

impl LatencySample {
    pub const fn new(milliseconds: f64) -> Self {
        Self { milliseconds }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_latency_sample() {
        let sample = LatencySample::new(2.5);
        assert_eq!(sample.milliseconds, 2.5);
    }
}

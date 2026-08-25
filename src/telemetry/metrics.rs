use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Default)]
pub struct Metrics {
    submitted: AtomicU64,
    completed: AtomicU64,
    rejected: AtomicU64,
    cache_hits: AtomicU64,
    cache_misses: AtomicU64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetricsSnapshot {
    pub submitted: u64,
    pub completed: u64,
    pub rejected: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl Metrics {
    pub const fn new() -> Self {
        Self {
            submitted: AtomicU64::new(0),
            completed: AtomicU64::new(0),
            rejected: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
        }
    }

    pub fn record_submitted(&self) {
        self.submitted.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_completed(&self) {
        self.completed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_rejected(&self) {
        self.rejected.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_cache_hit(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_cache_miss(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            submitted: self.submitted.load(Ordering::Relaxed),
            completed: self.completed.load(Ordering::Relaxed),
            rejected: self.rejected.load(Ordering::Relaxed),
            cache_hits: self.cache_hits.load(Ordering::Relaxed),
            cache_misses: self.cache_misses.load(Ordering::Relaxed),
        }
    }

    pub fn reset(&self) {
        self.submitted.store(0, Ordering::Relaxed);
        self.completed.store(0, Ordering::Relaxed);
        self.rejected.store(0, Ordering::Relaxed);
        self.cache_hits.store(0, Ordering::Relaxed);
        self.cache_misses.store(0, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn records_runtime_events() {
        let metrics = Metrics::new();

        metrics.record_submitted();
        metrics.record_submitted();
        metrics.record_completed();
        metrics.record_rejected();
        metrics.record_cache_hit();
        metrics.record_cache_miss();

        assert_eq!(
            metrics.snapshot(),
            MetricsSnapshot {
                submitted: 2,
                completed: 1,
                rejected: 1,
                cache_hits: 1,
                cache_misses: 1,
            }
        );
    }

    #[test]
    fn starts_empty() {
        let metrics = Metrics::new();

        assert_eq!(
            metrics.snapshot(),
            MetricsSnapshot {
                submitted: 0,
                completed: 0,
                rejected: 0,
                cache_hits: 0,
                cache_misses: 0,
            }
        );
    }

    #[test]
    fn reset_clears_all_counters() {
        let metrics = Metrics::new();

        metrics.record_submitted();
        metrics.record_completed();
        metrics.record_rejected();
        metrics.record_cache_hit();
        metrics.record_cache_miss();

        metrics.reset();

        assert_eq!(
            metrics.snapshot(),
            MetricsSnapshot {
                submitted: 0,
                completed: 0,
                rejected: 0,
                cache_hits: 0,
                cache_misses: 0,
            }
        );
    }

    #[test]
    fn supports_concurrent_recording() {
        let metrics = Arc::new(Metrics::new());
        let mut handles = Vec::new();

        for _ in 0..8 {
            let metrics = Arc::clone(&metrics);

            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    metrics.record_submitted();
                    metrics.record_completed();
                }
            }));
        }

        for handle in handles {
            handle.join().expect("metrics worker panicked");
        }

        let snapshot = metrics.snapshot();

        assert_eq!(snapshot.submitted, 800);
        assert_eq!(snapshot.completed, 800);
    }
}

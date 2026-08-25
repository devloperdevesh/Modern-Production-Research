use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex};

struct RuntimeState {
    active: usize,
}

pub struct BoundedExecutor {
    limit: usize,
    state: Mutex<RuntimeState>,
    available: Condvar,
    submitted: AtomicU64,
    completed: AtomicU64,
}

impl BoundedExecutor {
    pub fn new(limit: usize) -> Self {
        assert!(limit > 0, "executor limit must be greater than zero");

        Self {
            limit,
            state: Mutex::new(RuntimeState { active: 0 }),
            available: Condvar::new(),
            submitted: AtomicU64::new(0),
            completed: AtomicU64::new(0),
        }
    }

    pub const fn limit(&self) -> usize {
        self.limit
    }

    pub fn active(&self) -> usize {
        self.state
            .lock()
            .expect("runtime state lock poisoned")
            .active
    }

    pub fn submitted(&self) -> u64 {
        self.submitted.load(Ordering::Acquire)
    }

    pub fn completed(&self) -> u64 {
        self.completed.load(Ordering::Acquire)
    }

    pub fn execute<F, R>(&self, task: F) -> R
    where
        F: FnOnce() -> R,
    {
        self.acquire();

        self.submitted.fetch_add(1, Ordering::Release);

        let result = task();

        self.completed.fetch_add(1, Ordering::Release);
        self.release();

        result
    }

    fn acquire(&self) {
        let mut state = self.state.lock().expect("runtime state lock poisoned");

        while state.active >= self.limit {
            state = self
                .available
                .wait(state)
                .expect("runtime state lock poisoned");
        }

        state.active += 1;
    }

    fn release(&self) {
        let mut state = self.state.lock().expect("runtime state lock poisoned");

        state.active -= 1;

        self.available.notify_one();
    }
}

impl Default for BoundedExecutor {
    fn default() -> Self {
        Self::new(1)
    }
}

pub type SharedExecutor = Arc<BoundedExecutor>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn enforces_execution_limit() {
        let executor = BoundedExecutor::new(2);

        assert_eq!(executor.limit(), 2);
        assert_eq!(executor.active(), 0);

        let result = executor.execute(|| 42);

        assert_eq!(result, 42);
        assert_eq!(executor.active(), 0);
        assert_eq!(executor.submitted(), 1);
        assert_eq!(executor.completed(), 1);
    }

    #[test]
    fn executes_tasks_and_tracks_completion() {
        let executor = BoundedExecutor::new(4);

        let first = executor.execute(|| 10);
        let second = executor.execute(|| 20);

        assert_eq!(first + second, 30);
        assert_eq!(executor.submitted(), 2);
        assert_eq!(executor.completed(), 2);
        assert_eq!(executor.active(), 0);
    }

    #[test]
    fn blocks_until_capacity_is_available() {
        let executor = Arc::new(BoundedExecutor::new(1));
        let started = Arc::new(AtomicUsize::new(0));

        let first_executor = Arc::clone(&executor);
        let first_started = Arc::clone(&started);

        let first = thread::spawn(move || {
            first_executor.execute(|| {
                first_started.store(1, Ordering::Release);
                thread::sleep(Duration::from_millis(20));
            });
        });

        while started.load(Ordering::Acquire) == 0 {
            thread::yield_now();
        }

        let second_executor = Arc::clone(&executor);

        let second = thread::spawn(move || second_executor.execute(|| 99));

        first.join().expect("first worker panicked");

        assert_eq!(second.join().expect("second worker panicked"), 99);
        assert_eq!(executor.submitted(), 2);
        assert_eq!(executor.completed(), 2);
        assert_eq!(executor.active(), 0);
    }

    #[test]
    fn supports_shared_executor() {
        let executor: SharedExecutor = Arc::new(BoundedExecutor::new(2));

        let result = executor.execute(|| "runtime-ready");

        assert_eq!(result, "runtime-ready");
    }

    #[test]
    #[should_panic(expected = "executor limit must be greater than zero")]
    fn rejects_zero_limit() {
        let _ = BoundedExecutor::new(0);
    }
}

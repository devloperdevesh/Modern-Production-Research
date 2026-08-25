use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

pub struct Semaphore {
    permits: AtomicU64,
}

impl Semaphore {
    pub fn new(permits: u64) -> Self {
        Self {
            permits: AtomicU64::new(permits),
        }
    }

    pub fn available_permits(&self) -> u64 {
        self.permits.load(Ordering::Acquire)
    }

    pub fn try_acquire(&self) -> bool {
        loop {
            let current = self.permits.load(Ordering::Acquire);

            if current == 0 {
                return false;
            }

            if self
                .permits
                .compare_exchange_weak(current, current - 1, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return true;
            }
        }
    }

    pub fn acquire(&self) {
        while !self.try_acquire() {
            thread::yield_now();
        }
    }

    pub fn release(&self) {
        self.permits.fetch_add(1, Ordering::Release);
    }
}

pub struct Permit<'a> {
    semaphore: &'a Semaphore,
}

impl<'a> Permit<'a> {
    pub fn acquire(semaphore: &'a Semaphore) -> Self {
        semaphore.acquire();

        Self { semaphore }
    }
}

impl Drop for Permit<'_> {
    fn drop(&mut self) {
        self.semaphore.release();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn enforces_permit_limit() {
        let semaphore = Semaphore::new(2);

        assert!(semaphore.try_acquire());
        assert!(semaphore.try_acquire());
        assert!(!semaphore.try_acquire());

        semaphore.release();

        assert!(semaphore.try_acquire());
    }

    #[test]
    fn reports_available_permits() {
        let semaphore = Semaphore::new(3);

        assert_eq!(semaphore.available_permits(), 3);

        assert!(semaphore.try_acquire());
        assert_eq!(semaphore.available_permits(), 2);

        semaphore.release();
        assert_eq!(semaphore.available_permits(), 3);
    }

    #[test]
    fn permit_releases_on_drop() {
        let semaphore = Semaphore::new(1);

        {
            let _permit = Permit::acquire(&semaphore);
            assert_eq!(semaphore.available_permits(), 0);
        }

        assert_eq!(semaphore.available_permits(), 1);
    }

    #[test]
    fn supports_concurrent_acquisition() {
        let semaphore = Arc::new(Semaphore::new(4));
        let mut handles = Vec::new();

        for _ in 0..8 {
            let semaphore = Arc::clone(&semaphore);

            handles.push(thread::spawn(move || {
                if semaphore.try_acquire() {
                    thread::sleep(Duration::from_millis(1));
                    semaphore.release();
                    true
                } else {
                    false
                }
            }));
        }

        let acquired = handles
            .into_iter()
            .filter_map(|handle| handle.join().ok())
            .filter(|acquired| *acquired)
            .count();

        assert!(acquired > 0);
        assert_eq!(semaphore.available_permits(), 4);
    }
}

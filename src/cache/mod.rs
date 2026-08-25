use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct CacheEntry<T> {
    value: T,
    expires_at: Instant,
}

pub struct LocalCache<T> {
    entries: HashMap<String, CacheEntry<T>>,
}

impl<T> LocalCache<T> {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: T, ttl: Duration) {
        self.entries.insert(
            key,
            CacheEntry {
                value,
                expires_at: Instant::now() + ttl,
            },
        );
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.entries.get(key).and_then(|entry| {
            if Instant::now() < entry.expires_at {
                Some(&entry.value)
            } else {
                None
            }
        })
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl<T> Default for LocalCache<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_and_reads_value() {
        let mut cache = LocalCache::new();

        assert!(cache.is_empty());

        cache.insert("request".to_string(), "response", Duration::from_secs(60));

        assert_eq!(cache.get("request"), Some(&"response"));
        assert!(!cache.is_empty());
        assert_eq!(cache.len(), 1);
    }
}

pub mod local_cache;

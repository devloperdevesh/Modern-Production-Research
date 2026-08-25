use std::collections::HashMap;
use std::hash::Hash;
use std::sync::RwLock;

pub struct LocalCache<K, V> {
    entries: RwLock<HashMap<K, V>>,
    capacity: usize,
}

impl<K, V> LocalCache<K, V>
where
    K: Eq + Hash,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            capacity,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.entries.read().expect("cache read lock poisoned").len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn insert(&self, key: K, value: V) -> bool {
        let mut entries = self.entries.write().expect("cache write lock poisoned");

        if !entries.contains_key(&key) && entries.len() >= self.capacity {
            return false;
        }

        entries.insert(key, value);
        true
    }

    pub fn get(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        self.entries
            .read()
            .expect("cache read lock poisoned")
            .get(key)
            .cloned()
    }

    pub fn remove(&self, key: &K) -> Option<V> {
        self.entries
            .write()
            .expect("cache write lock poisoned")
            .remove(key)
    }

    pub fn clear(&self) {
        self.entries
            .write()
            .expect("cache write lock poisoned")
            .clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn stores_and_reads_value() {
        let cache = LocalCache::new(10);

        assert!(cache.insert("key", "value"));
        assert_eq!(cache.get(&"key"), Some("value"));
    }

    #[test]
    fn updates_existing_key_without_growing_cache() {
        let cache = LocalCache::new(1);

        assert!(cache.insert("key", "first"));
        assert!(cache.insert("key", "second"));

        assert_eq!(cache.len(), 1);
        assert_eq!(cache.get(&"key"), Some("second"));
    }

    #[test]
    fn rejects_new_entry_when_capacity_is_full() {
        let cache = LocalCache::new(1);

        assert!(cache.insert("first", 1));
        assert!(!cache.insert("second", 2));

        assert_eq!(cache.len(), 1);
        assert_eq!(cache.get(&"first"), Some(1));
    }

    #[test]
    fn removes_and_clears_entries() {
        let cache = LocalCache::new(10);

        cache.insert("a", 1);
        cache.insert("b", 2);

        assert_eq!(cache.remove(&"a"), Some(1));
        assert_eq!(cache.get(&"a"), None);

        cache.clear();

        assert!(cache.is_empty());
    }

    #[test]
    fn supports_concurrent_access() {
        let cache = Arc::new(LocalCache::new(100));

        let mut handles = Vec::new();

        for index in 0..8 {
            let cache = Arc::clone(&cache);

            handles.push(thread::spawn(move || {
                assert!(cache.insert(index, index * 10));
            }));
        }

        for handle in handles {
            handle.join().expect("worker thread panicked");
        }

        assert_eq!(cache.len(), 8);
        assert_eq!(cache.get(&7), Some(70));
    }
}

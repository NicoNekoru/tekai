//! Byte- and entry-bounded retention. Values in use are owned by callers, not
//! pinned in the cache, so eviction never invalidates an active reader.

use std::collections::HashMap;
use std::hash::Hash;

pub(crate) struct BudgetCache<K, V> {
    entries: HashMap<K, (V, usize, u64)>,
    bytes: usize,
    budget: usize,
    limit: usize,
    clock: u64,
}

impl<K: Eq + Hash + Clone, V> BudgetCache<K, V> {
    pub(crate) fn new(budget: usize, limit: usize) -> Self {
        Self {
            entries: HashMap::new(),
            bytes: 0,
            budget,
            limit,
            clock: 0,
        }
    }

    pub(crate) fn get(&mut self, key: &K) -> Option<&V> {
        self.clock = self.clock.saturating_add(1);
        let entry = self.entries.get_mut(key)?;
        entry.2 = self.clock;
        Some(&entry.0)
    }

    pub(crate) fn remove(&mut self, key: &K) -> Option<V> {
        let (value, bytes, _) = self.entries.remove(key)?;
        self.bytes -= bytes;
        Some(value)
    }

    pub(crate) fn keys(&self) -> impl Iterator<Item = &K> {
        self.entries.keys()
    }

    pub(crate) fn peek(&self, key: &K) -> Option<&V> {
        self.entries.get(key).map(|entry| &entry.0)
    }

    pub(crate) fn insert(&mut self, key: K, value: V, bytes: usize) {
        self.remove(&key);
        if bytes > self.budget || self.limit == 0 {
            return;
        }
        while self.bytes > self.budget - bytes || self.entries.len() >= self.limit {
            let oldest = self
                .entries
                .iter()
                .min_by_key(|(_, entry)| entry.2)
                .map(|(key, _)| key.clone())
                .expect("nonempty over-budget cache");
            self.remove(&oldest);
        }
        self.clock = self.clock.saturating_add(1);
        self.bytes += bytes;
        self.entries.insert(key, (value, bytes, self.clock));
    }

    #[cfg(test)]
    pub(crate) fn retained_bytes(&self) -> usize {
        self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn budgets_admission_replacement_and_lru() {
        let mut cache = BudgetCache::new(10, 2);
        cache.insert(1, "one", 5);
        cache.insert(2, "two", 5);
        assert_eq!(cache.get(&1), Some(&"one"));
        cache.insert(3, "three", 5);
        assert!(cache.get(&2).is_none());
        cache.insert(1, "replacement", 3);
        assert_eq!(cache.retained_bytes(), 8);
        cache.insert(4, "oversized", 11);
        assert_eq!(cache.retained_bytes(), 8);
        cache.insert(5, "tiny", 0);
        assert_eq!(cache.keys().count(), 2);
    }
}

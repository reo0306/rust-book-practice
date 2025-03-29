use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

pub struct SimpleCache<K, V> {
    pub capacity: usize,
    pub map: HashMap<K, V>,
    pub order: VecDeque<K>,
}

impl<K, V> SimpleCache<K, V>
where
    K: Eq + Hash + Clone
{
    pub fn new(capacity: usize) -> Self {
        Self { 
            capacity,
            map: HashMap::with_capacity(capacity),
            order: VecDeque::with_capacity(capacity),
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.order.retain(|k| k != &key);
        } else if self.map.len() >= self.capacity {
            if let Some(oldest) = self.order.pop_back() {
                self.map.remove(&oldest);
            }
        }
        self.order.push_front(key.clone());
        self.map.insert(key, value);
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.order.retain(|k| k != key);
            self.order.push_front(key.clone());
            self.map.get(key)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_simple_cache() {
        let mut cache = SimpleCache::new(2);

        cache.put(1, "one");
        cache.put(2, "two");
        assert_eq!(cache.get(&1), Some(&"one"));
        assert_eq!(cache.get(&2), Some(&"two"));

        cache.put(3, "three");
        assert_eq!(cache.get(&1), None); 
        assert_eq!(cache.get(&2), Some(&"two"));
        assert_eq!(cache.get(&3), Some(&"three"));

        cache.get(&2);
        cache.put(4, "four");
        assert_eq!(cache.get(&3), None); 
        assert_eq!(cache.get(&2), Some(&"two")); 
        assert_eq!(cache.get(&4), Some(&"four")); 
    }
}

use std::collections::HashMap;
use std::hash::Hash;

pub struct SimpleCache<K, V> {
    pub cache: HashMap<K, V>,
}

impl<K, V> SimpleCache<K, V>
where
    K: Eq + Hash
{
    pub fn new(capacity: usize) -> Self {
        Self { 
            cache: HashMap::with_capacity(capacity)
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        self.cache.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.cache.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_simple_cache() {
        let mut cache = SimpleCache::new(1);
        cache.put(1, 1);
        assert_eq!(&1, cache.get(&1).unwrap());
    }
}

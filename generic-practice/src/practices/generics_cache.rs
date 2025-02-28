use std::collections::HashMap;
use std::hash::Hash;

pub struct Cache<K, V> {
    pub storage: HashMap<K, V>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + PartialEq + Hash,
{

    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: K, value: V) {
        self.storage.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.storage.get(key)
    }

    pub fn has(&self, key: &K) -> bool {
        self.storage.contains_key(key)
    }
}
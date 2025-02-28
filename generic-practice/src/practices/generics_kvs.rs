use std::collections::HashMap;
use std::hash::Hash;

pub struct KeyValueStore<K, V> {
    pub store: HashMap<K, V>,
}

impl<K, V> KeyValueStore<K, V>
where
    K: Eq + PartialEq + Hash,
{
    pub fn new() -> Self {
        Self { store: HashMap::new() }
    }

    pub fn set(&mut self, key: K, value: V) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.store.get(key)
    }

    pub fn has(&self, key: &K) -> bool {
        self.store.contains_key(key)
    }
}
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Item {
    pub id: i32,
    pub categroy: String,
    pub value: i32,
}

pub fn group_by_key<T, F>(array: Vec<Item>, key_extractor: F) -> HashMap<T, Vec<Item>>
where
    T: Eq + Hash + Debug + Clone,
    F: Fn(&Item) -> T,
{
    let mut result: HashMap<T, Vec<Item>> = HashMap::new();

    for item in array {
        let key = key_extractor(&item);
        result.entry(key).or_insert(Vec::new()).push(item);
    }

    result
}
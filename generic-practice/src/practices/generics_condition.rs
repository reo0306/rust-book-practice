pub fn filter_by_condition<T, F>(array: &[T], condition: F) -> Vec<T>
where
    T: Clone
    F: Fn(&T) -> bool
{
    array.iter().filter(|&x| condition(x)).cloned().collect()
}
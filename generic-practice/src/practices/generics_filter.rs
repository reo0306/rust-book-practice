pub fn filter_array<F>(array: &[i32], predicate: F) -> Vec<i32>
where F: Fn(i32) -> bool,
{
    array.iter().copied().filter(|&x| predicate(x)).collect()
}
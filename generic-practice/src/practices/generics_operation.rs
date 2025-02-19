pub fn apply_operation<F>(array: &[i32], operation: F) -> Vec<i32>
where F: Fn(i32) -> i32,
{
    array.iter().map(|&x| operation(x)).collect()
}
/*
pub fn apply_operation(mut array: [i32; 5], operation: fn(i32) -> i32) -> [i32; 5] {
    for i in 0..array.len() {
        array[i] = operation(array[i]);
    }
    array
}
*/
pub fn apply_operation2(a: i32, b: i32, operations: Vec<fn(i32, i32) -> i32>) -> Vec<i32>
{
    operations.iter().map(|&x| x(a, b)).collect::<Vec<_>>()
}
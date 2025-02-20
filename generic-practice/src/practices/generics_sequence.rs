pub fn generate_sequence<F>(start: i32, end: i32, step: i32, operation: F) -> Vec<i32>
where F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for i in (start..=end).step_by(step.abs() as usize) {
        result.push(operation(i));
    }

    result
}
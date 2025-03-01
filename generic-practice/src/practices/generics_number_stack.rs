use std::iter::Sum;
use std::convert::Into;

pub struct NumberStack<T> {
    pub stack: Vec<T>,
}

impl<T> NumberStack<T> 
where T: Copy + Into<f64> + Sum<T>
{
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn sum(&self) -> f64 {
        self.stack.iter().copied().map(|x| x.into()).sum()
    }
}
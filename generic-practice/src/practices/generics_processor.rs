
pub trait Processor<T> {
    fn process(&self, value: T) -> T;
}

#[derive(Debug)]
pub struct Doubler;

impl Processor<i32> for Doubler{
    fn process(&self, value: i32) -> i32 {
        value * 2
    }
}

#[derive(Debug)]
pub struct Reverser;

impl Processor<String> for Reverser {
    fn process(&self, value: String) -> String {
        value.chars().rev().collect::<String>()
    }
}

pub fn get_int_processor() -> Box<dyn Processor<i32>> {
    Box::new(Doubler)
}

pub fn get_str_processor() -> Box<dyn Processor<String>> {
    Box::new(Reverser)
}
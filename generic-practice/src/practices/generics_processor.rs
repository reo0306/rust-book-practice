pub trait Processor<T> {
    fn process(&self, value: T) -> T;
}

#[derive(Debug)]
pub struct IntProcessor;

impl Processor<i32> for IntProcessor {
    fn process(&self, value: i32) -> i32 {
        value * 2
    }
}

#[derive(Debug)]
pub struct StrProcesor;

impl Processor<String> for StrProcesor {
    fn process(&self, value: String) -> String {
        value.to_uppercase()
    }
}

pub fn get_processor(data: &str) -> Box<dyn Processor<String>> {
    if data == "int" {
        Box::new(IntProcessor) as Box<dyn Processor<String>>
    } else {
        Box::new(StrProcesor)
    }
}
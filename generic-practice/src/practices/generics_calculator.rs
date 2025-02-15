pub trait Calculator {
    fn calculate(&self, a: i32, b: i32) -> i32;
}

pub struct Adder;

impl Calculator for Adder {
    fn calculate(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

pub struct Multiplier;

impl Calculator for Multiplier {
    fn calculate(&self, a: i32, b: i32) -> i32 {
        a * b
    }
}

pub fn get_calculator(value: &str) -> Box<dyn Calculator> {
    if value == "add" {
        Box::new(Adder)
    } else {
        Box::new(Multiplier)
    }
}
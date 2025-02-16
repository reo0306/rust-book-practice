pub trait Converter<T> {
    fn convert(&self, value: T) -> T;
}

#[derive(Debug)]
pub struct CelsiusToFahrenheit;

impl Converter<i32> for CelsiusToFahrenheit {
    fn convert(&self, value: i32) -> i32 {
        value * 9 / 5 + 32
    }
}

#[derive(Debug)]
pub struct UpperCaseConverter;

impl Converter<String> for UpperCaseConverter {
    fn convert(&self, value: String) -> String {
        value.to_uppercase()
    }
}

pub fn get_int_converter() -> Box<impl Converter<i32>> {
    Box::new(CelsiusToFahrenheit)
}

pub fn get_str_converter() -> Box<impl Converter<String>> {
    Box::new(UpperCaseConverter)
}
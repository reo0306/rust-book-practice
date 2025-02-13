use num_format::{Locale, ToFormattedString};

pub trait Formatter<T> {
    fn format(&self, value: T) -> String;
}

#[derive(Debug)]
pub struct UpperCaseFormatter;

impl<T: ToString> Formatter<T> for UpperCaseFormatter {
    fn format(&self, value: T) -> String {
        value.to_string().to_uppercase()
    }
}

#[derive(Debug)]
pub struct NumberFormatter;

impl<T: num_format::ToFormattedStr> Formatter<T> for NumberFormatter {
    fn format(&self, value: T) -> String {
        value.to_formatted_string(&Locale::en)
    }
}

pub fn print_formatted<T>(formatter: &dyn Formatter<T>, value: T) {
    println!("{:?}", formatter.format(value));
}

pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_celsius_to_fahrenheit() {
        let result = celsius_to_fahrenheit(0.0);
        assert_eq!(result, 32.0);
    }

    #[test]
    fn it_fahrenheit_to_celsius() {
        let result = fahrenheit_to_celsius(212.0);
        assert_eq!(result, 100.0);
    }
}

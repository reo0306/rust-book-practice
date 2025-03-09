pub struct Celsius(pub f64);

impl Celsius {
    pub fn to_fahrenheit(&self) -> f64 {
        (self.0 * 9.0 / 5.0) + 32.0
    }

    pub fn get_value(&self) -> f64 {
        self.0
    }
}

pub struct Fahrenheit(pub f64);

impl Fahrenheit {
    pub fn to_celsius(&self) -> f64 {
        (self.0 - 32.0) * 5.0 / 9.0
    }

    pub fn get_value(&self) -> f64 {
        self.0
    }
}


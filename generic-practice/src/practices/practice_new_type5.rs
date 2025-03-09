pub struct Usd(pub f64);

impl Usd {
    pub fn to_jpy(&self) -> f64 {
        self.0 * 150.0
    }

    pub fn get_amount(&self) -> f64 {
        self.0
    }
}

pub struct Jpy(pub f64);

impl Jpy {
    pub fn to_usd(&self) -> f64 {
        self.0 / 150.0
    }

    pub fn get_amount(&self) -> f64 {
        self.0
    }
}
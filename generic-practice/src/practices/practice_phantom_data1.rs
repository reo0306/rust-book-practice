use std::marker::PhantomData;

pub struct UsdEx;
pub struct JpyEx;

pub struct Amount<T> {
    value: f64,
    _phantom: PhantomData<T>,
}

impl<T> Amount<T> {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}

impl Amount<JpyEx> {
    pub fn to_usd(&self) -> Amount<UsdEx> {
        Amount::new(self.value / 150.0)
    }
}

impl Amount<UsdEx> {
    pub fn to_jpy(&self) -> Amount<JpyEx> { 
        Amount::new(self.value * 150.0)
    }
}
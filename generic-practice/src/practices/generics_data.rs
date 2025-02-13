pub struct BoxValue<T> {
    value: T,
}

impl<T> BoxValue<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn get(&self) -> &T {
        &self.value
    }
}
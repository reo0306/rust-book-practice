type Length = f64;
type Width = f64;

pub struct Rectangle {
    length: Length,
    width: Width,
}

impl Rectangle{
    pub fn new(length: f64, width: f64) -> Self {
        Self { length, width }
    }

    pub fn area(&self) -> f64 {
        &self.length * &self.width
    }
    
    pub fn perimeter(&self) -> f64 {
        2.0 * (&self.length + &self.width)
    }
}
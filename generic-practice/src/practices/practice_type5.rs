use std::f64::consts::PI;

type Dimension = f64;

pub trait Shape {
    fn area(&self) -> Dimension;
}

pub struct DimensionCircle {
    radius: Dimension,
}

impl DimensionCircle {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Shape for DimensionCircle {
    fn area(&self) -> f64 {
        PI * &self.radius.powf(2.0)
    }
}

pub struct DimensionRectangle {
    width: Dimension,
    height: Dimension,
}

impl DimensionRectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl Shape for DimensionRectangle {
    fn area(&self) -> f64 {
        &self.width * &self.height
    }
}
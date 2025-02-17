use std::f64::consts::PI;

pub trait Shape<T> {
    fn area(&self) -> T;
}

pub struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}
impl Shape<i32> for Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

pub struct Circle {
    radius: i32,
}

impl Circle {
    fn new(radius: i32) -> Self {
        Self { radius }
    }
}
impl Shape<f64> for Circle {
    fn area(&self) -> f64 {
        (self.radius as f64).powi(2) * PI
    }
}

pub fn get_int_shape() -> Box<dyn Shape<i32>> {
    Box::new(Rectangle::new(4, 5))
}

pub fn get_float_shape() -> Box<dyn Shape<f64>> {
    Box::new(Circle::new(3))
}
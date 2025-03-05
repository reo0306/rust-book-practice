type Coordinate = f64;

pub struct Point3D {
    x: Coordinate,
    y: Coordinate,
    z: Coordinate,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn distance_from_origin(&self) -> f64 {
        let val = &self.x.powf(2.0) + &self.y.powf(2.0) + &self.z.powf(2.0);
        val.sqrt()
    }
}
pub enum TransportMode {
    Car,
    Bicycle,
    Walking,
}

pub struct Travel {
    mode: TransportMode,
    distance: f64,
}

impl Travel {
    pub fn new(mode: TransportMode, distance: f64) -> Self {
        Self {
            mode,
            distance,
        }
    }

    pub fn calcuate_time(&self) -> f64 {
        let speed = match &self.mode {
            TransportMode::Car => 100.0,
            TransportMode::Bicycle => 20.0,
            TransportMode::Walking => 5.0,
        };

        &self.distance / speed
    }
}
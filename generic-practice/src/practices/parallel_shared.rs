pub struct ParallelSharedCounter {
    pub value: i32,
}

impl ParallelSharedCounter {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}
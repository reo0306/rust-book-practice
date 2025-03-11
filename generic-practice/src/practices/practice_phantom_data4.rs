use std::marker::PhantomData;

pub struct Pending;
pub struct Processed;
pub struct Shipped;

pub struct Order<T> {
    state: String,
    _phantom: PhantomData<T>,
}

impl<T> Order<T> {
    pub fn new() -> Self {
        Self { state: "Pending".to_string(), _phantom: PhantomData }
    }

    pub fn get_state(&self) -> String {
        self.state.clone()
    }
}

impl Order<Pending> {
    pub fn process(self) -> Order<Processed> {
        Order { state: "Processed".to_string(), _phantom: PhantomData }
    }
}

impl Order<Processed> {
    pub fn ship(self) -> Order<Shipped> {
        Order { state: "Shipped".to_string(), _phantom: PhantomData }
    }
}
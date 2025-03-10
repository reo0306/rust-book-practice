use std::marker::PhantomData;

pub struct Connected;
pub struct Disconnected;

pub struct Database<T> {
    _phantom: PhantomData<T>,
}

impl<T> Database<T> {
    pub fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl Database<Disconnected> {
    pub fn connect(self) -> Database<Connected> {
        println!("Connected to the database");
        Database { _phantom: PhantomData }
    }
}

impl Database<Connected> {
    pub fn fetch_data(&self) -> String {
        "Data from database".to_string()
    }
}
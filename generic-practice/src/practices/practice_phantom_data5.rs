use std::marker::PhantomData;
use std::fs::File;
use std::io::{Read, Result};

pub struct Open;
pub struct Closed;

pub struct PhantomDataFile<T> {
    handle: Option<File>,
    _phantom: PhantomData<T>,
}

impl PhantomDataFile<Closed> {
    pub fn new() -> Self {
        Self { handle: None, _phantom: PhantomData }
    }

    pub fn open(self, path: &str) -> Result<PhantomDataFile<Open>> {
        let file = File::open(path)?;
        Ok(
            PhantomDataFile { handle: Some(file), _phantom: PhantomData }
        )
    }
}

impl PhantomDataFile<Open> {
    pub fn read(&mut self) -> Result<String> {
        let mut contents = String::new();
        if let Some(file) = &mut self.handle {
            file.read_to_string(&mut contents)?;
        }
        Ok(contents)
    }

    pub fn close(self) -> PhantomDataFile<Closed> {
        PhantomDataFile {
            handle: None,
            _phantom: PhantomData
        }
    }
}
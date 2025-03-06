use std::fs::File;
use std::io::Write;

type LogMessage = String;

pub trait Logger {
    fn log(&self, message: LogMessage);
}

pub struct ConsoleLogger;

impl ConsoleLogger{
    pub fn new() -> Self {
        Self
    }
}

impl Logger for ConsoleLogger {
    fn log(&self, message: String) {
        println!("Console Log: {}", message);
    }
}

pub struct FileLogger;

impl FileLogger {
    pub fn new() -> Self {
        Self
    }
}

impl Logger for FileLogger {
    fn log(&self, message: String) {
        let path = "log.txt";
        let mut file = File::create(path).expect("file not found");
        writeln!(file, "{}", message).expect("error write");
    }
}
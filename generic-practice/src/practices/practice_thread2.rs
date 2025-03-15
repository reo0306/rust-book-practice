use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub struct PracticeThread2;

impl PracticeThread2 {
    pub fn exec() {
        let data = Arc::new(Mutex::new());
        let mut handles: Vec<JoinHandle<i32>> = vec![];

        for _ in 0..5 {
            let data_clone = Arc::clone(&data);
            handles.push(thread::spawn(move || {
                data_clone.iter().sum::<i32>()
            }));
        }

        let mut result = vec![];
        for handle in handles {
            result.push(handle.join().unwrap());
        }

        for res in result {
            println!("Final Counter: {}", res);
        }
    }
}
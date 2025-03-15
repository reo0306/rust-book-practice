use std::sync::{Arc, Mutex};
use std::thread;

pub struct PracticeThread3;

impl PracticeThread3 {
    pub fn exec() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..3 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut count = counter_clone.lock().unwrap();
                for i in 1..=50 {
                    if i % 2 != 0 {
                        *count += i;
                    }
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Final Counter: {}", *counter.lock().unwrap());
    }
}
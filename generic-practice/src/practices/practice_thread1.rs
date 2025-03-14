use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

pub struct PracticeThread1;

impl PracticeThread1 {
    pub fn exec() {
        let data = Arc::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let mut handles: Vec<JoinHandle<i32>> = vec![];

        for _ in 0..2 {
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
            println!("Total sum: {}", res);
        }
    }
}
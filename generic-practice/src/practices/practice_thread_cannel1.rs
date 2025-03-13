use std::thread;
use std::sync::mpsc;
use rand::Rng;

pub struct PracticeThreadChannel1;

impl PracticeThreadChannel1 {
    pub fn exec() {
        let (tx, rx) = mpsc::channel();

        let mut handles = vec![];

        for _ in 0..3 {
            let tx = tx.clone();

            let handle = thread::spawn(move || {
                let value = rand::thread_rng().gen_range(1..=10);
                println!("スレッドが生成した値：{}", value);
                tx.send(value).unwrap();
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let mut total = 0;
        for _ in 0..3 {
            total += rx.recv().unwrap();
        }

        println!("合計値： {}", total);
    }
}
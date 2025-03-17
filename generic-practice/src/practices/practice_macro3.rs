macro_rules! repeat {
    ($x:expr, $y:block) => {
        for i in 1..=$x {
            println!("Iteration: {}", i);
            $y
        }
    };
}

pub struct PracticeMacro3;

impl PracticeMacro3 {
    pub fn exec() {
        repeat!(3, { println!("Hello, Rust") });
    }
}
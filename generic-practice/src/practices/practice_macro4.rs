macro_rules! array_sum {
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),*) => {
        $x $(+ $y)*
    };
}

pub struct PracticeMacro4;

impl PracticeMacro4 {
    pub fn exec() {
        println!("{}", array_sum!(1, 2, 3, 4, 5));
        println!("{}", array_sum!(10, 20, 30));
        println!("{}", array_sum!(100));
    }
}
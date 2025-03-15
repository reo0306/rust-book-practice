macro_rules! calc {
    (+, $x:expr, $y:expr) => {
        $x + $y
    };
    (-, $x:expr, $y:expr) => {
        $x - $y
    };
    (*, $x:expr, $y:expr) => {
        $x * $y
    };
    (/, $x:expr, $y:expr) => {
        $x / $y
    };
}

pub struct PracticeMacro1;

impl PracticeMacro1 {
    pub fn exec() {
        println!("{}", calc!(+, 3, 5));
        println!("{}", calc!(-, 10, 4));
        println!("{}", calc!(*, 6, 7));
        println!("{}", calc!(/, 20, 4));
    }
}
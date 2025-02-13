use generic_practice::practices::{
    generics::swap,
    generics_data::BoxValue,
    generics_trait::{UpperCaseFormatter, NumberFormatter, print_formatted},
};

fn main() {
    let (x, y)= swap(1, "hello".to_string());
    println!("x:{}, y:{}", x, y);

    let int_box = BoxValue::new(42);
    let string_box = BoxValue::new("Rust");
    println!("{}", int_box.get());
    println!("{}", string_box.get());

    print_formatted(&UpperCaseFormatter, "rust".to_string());
    print_formatted(&NumberFormatter, 1000);
}

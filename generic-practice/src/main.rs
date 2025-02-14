use generic_practice::practices::{
    generics::swap,
    generics_data::BoxValue,
    generics_trait::{UpperCaseFormatter, NumberFormatter, print_formatted},
    generics_sort::get_sorter,
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

    let mut values = [5, 3, 8, 1, 2];
    let sorter = get_sorter(true);
    sorter.sorter(&mut values);
    println!("昇順: {:?}", values);

    let mut values = [5, 3, 8, 1, 2];
    let sorter = get_sorter(false);
    sorter.sorter(&mut values);
    println!("降順: {:?}", values);
}

use generic_practice::practices::{
    generics::swap,
    generics_data::BoxValue,
    generics_trait::{UpperCaseFormatter, NumberFormatter, print_formatted},
    generics_sort::get_sorter,
    generics_processor::{get_int_processor, get_str_processor},
    generics_calculator::get_calculator,
    generics_converter::{get_int_converter, get_str_converter, Converter},
    generics_rectangle::{get_int_shape, get_float_shape},
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

    let processor = get_int_processor();
    println!("{:?}", processor.process(10));

    let processor = get_str_processor();
    println!("{:?}", processor.process("hello".to_string()));

    let calc = get_calculator("add");
    println!("{:?}", calc.calculate(3, 5));

    let calc = get_calculator("mul");
    println!("{:?}", calc.calculate(3, 5));

    let converter = get_int_converter();
    println!("{:?}", converter.convert(25));

    let converter = get_str_converter();
    println!("{:?}", converter.convert("rust".to_string()));

    let shape = get_int_shape();
    println!("{}", shape.area());

    let shape = get_float_shape();
    println!("{}", shape.area());
}

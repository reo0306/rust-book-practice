
use generic_practice::practices::{
    generics::swap,
    generics_data::BoxValue,
    generics_trait::{UpperCaseFormatter, NumberFormatter, print_formatted},
    generics_sort::get_sorter,
    generics_processor::{get_int_processor, get_str_processor},
    generics_calculator::get_calculator,
    generics_converter::{get_int_converter, get_str_converter, Converter},
    generics_rectangle::{get_int_shape, get_float_shape},
    generics_operation::apply_operation,
    generics_filter::filter_array,
    generics_apply::apply_operation2,
    generics_sequence::generate_sequence,
    generics_group_by::{group_by_key, Item},
    generics_condition::filter_by_condition,
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

    let numbers = vec![1,2,3,4,5];

    let dobule = |n: i32| n * 2;
    let increment = |n: i32| n + 1;

    let dobule_result = apply_operation(&numbers, dobule);
    println!("{:?}", dobule_result);
    let increment_result = apply_operation(&numbers, increment);
    println!("{:?}", increment_result);

    let numbers2 = vec![1,2,3,4,5,6,7,8,9,10];
    let is_even = |x: i32| x % 2 == 0;
    let is_odd = |x: i32| x %2 != 0;

    let even_numbers = filter_array(&numbers2, is_even);
    println!("{:?}", even_numbers);
    let odd_numbers = filter_array(&numbers2, is_odd);
    println!("{:?}", odd_numbers);

    let operations = vec![|a: i32, b: i32| a + b, |a: i32, b: i32| a * b];
    let results = apply_operation2(3, 4, operations);
    println!("{:?}", results);

    let double = |x: i32| x * 2;
    let square = |x: i32| x * x;

    let double_sequence = generate_sequence(1, 10, 2, double);
    println!("{:?}", double_sequence);
    let square_sequence = generate_sequence(1, 10, 2, square);
    println!("{:?}", square_sequence);

    let data = vec![
        Item {
            id: 1,
            categroy: "A".to_string(),
            value: 100,
        },
        Item {
            id: 2,
            categroy: "B".to_string(),
            value: 200,
        },
        Item {
            id: 3,
            categroy: "A".to_string(),
            value: 150,
        },
        Item {
            id: 4,
            categroy: "B".to_string(),
            value: 250,
        },
        Item {
            id: 5,
            categroy: "C".to_string(),
            value: 300,
        },
    ];

    let group_by_category = group_by_key(data.clone(), |item| item.categroy.clone());
    println!("Categroy {:?}", group_by_category);

    let group_by_id = group_by_key(data.clone(), |item| item.id);
    println!("Id {:?}", group_by_id);

    let group_by_value = group_by_key(data.clone(), |item| item.value);
    println!("Value {:?}", group_by_value);

    let data = vec![1,2,3,4,5,6,7,8,9,10];

    let even_numbers = filter_by_condition(&data, |&x| x % 2 == 0);
    println!("{:?}", even_numbers);

    let greater_than_five = filter_by_condition(&data, |&x| x >= 5);
    println!("{:?}", greater_than_five);
}

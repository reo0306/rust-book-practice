use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::collections::HashMap;

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
    generics_prefix::filter_by_prefix,
    generics_kvs::KeyValueStore,
    generics_stack::Stack,
    generics_cache::Cache,
    generics_number_stack::NumberStack,
    parallel_data::ParallelData,
    parallel_counter::ParallelCounter,
    parallel_shared::ParallelSharedCounter,
    parallel_arc_mutex::MultiSharedCounter,
    parallel_node::LinkedList,
    parallel_bst::BinaryTree,
    parallel_queue::SharedQueue,
    parallel_counter_ex::CounterEx,
    parallel_stack::ParallelStack,
    practice_type1::User,
    practice_type2::Product,
    practice_type3::Rectangle,
    practice_type4::Point3D,
    practice_type5::{DimensionCircle, DimensionRectangle, Shape},
    practice_type6::{Logger, ConsoleLogger, FileLogger},
    practice_type7::{Payment, CreditCardPayment, PaypalPayment},
    practice_enum1::{TransportMode, Travel},
    practice_enum2::{PaymentMethod, PaymentEx},
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

    let words = vec!["apple", "banana", "apricot", "blueberry", "avocado"];

    let filtered = filter_by_prefix(&words, "a");
    println!("{:?}", filtered);

    let data = Rc::new(ParallelData { value: 42 });
    let a = data.clone();
    let b = data.clone();
    println!("{}", b.value);

    let counter = Rc::new(RefCell::new(ParallelCounter { value: 0}));
    let counter1 = counter.clone();
    let counter2 = counter.clone();
    counter1.borrow_mut().value = 1;
    println!("{}", counter1.borrow().value);
    counter2.borrow_mut().value = 2;
    println!("{}", counter2.borrow().value);

    let shared_counter = Rc::new(RefCell::new(ParallelSharedCounter::new()));
    let shared_counter1 = shared_counter.clone();
    let shared_counter2 = shared_counter.clone();

    shared_counter1.borrow_mut().increment();
    shared_counter2.borrow_mut().increment();

    println!("{}", shared_counter.borrow().get_value());

    let multi_counter = Arc::new(Mutex::new(MultiSharedCounter::new()));

    let mut threads = Vec::new();

    for _ in 0..5 {
        let multi_counter_clone = Arc::clone(&multi_counter);
        let handle = thread::spawn(move || {
            let mut shared = multi_counter_clone.lock().unwrap();
            shared.increment();
        });
        threads.push(handle);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let result = multi_counter.lock().unwrap();
    println!("Final Counter Value: {}", result.get_value());

    let multi_counter2 = Arc::new(AtomicI32::new(0));

    let mut threads2 = Vec::new();

    for _ in 0..5 {
        let multi_counter_clone2 = Arc::clone(&multi_counter2);
        let handle2 = thread::spawn(move || {
            multi_counter_clone2.fetch_add(10, Ordering::SeqCst);
        });
        threads2.push(handle2);
    }

    for thread2 in threads2 {
        thread2.join().unwrap();
    }

    println!("Final Counter Value2: {}", multi_counter2.load(Ordering::SeqCst));

    let mut stack = Stack::new();
    stack.push(10);
    stack.push(20);
    println!("{:?}", stack.pop().unwrap());
    println!("{}", stack.len());

    let mut kv = KeyValueStore::new();
    kv.set("name".to_string(), "Alice".to_string());
    println!("{}", kv.get(&"name".to_string()).unwrap());
    println!("{}", kv.has(&"name".to_string()));

    let cache = Rc::new(RefCell::new(Cache::<String, HashMap<String, String>>::new()));
    let cache1 = cache.clone();
    let cache2 = cache.clone();

    {
        let mut cache_data = cache1.borrow_mut();
        let entry1 = cache_data.storage.entry("user".to_string()).or_insert(HashMap::new());
        entry1.insert("name".to_string(), "Alice".to_string());
    }
    //let mut cache_data = cache1.borrow_mut();
    //let entry1 = cache_data.storage.entry("user".to_string()).or_insert(HashMap::new());
    //entry1.insert("name".to_string(), "Alice".to_string());
    //cache_data1.borrow_mut().set("user".to_string(), data1.clone());

    {
        let mut cache_data = cache2.borrow_mut();
        let entry2 = cache_data.storage.entry("user".to_string()).or_insert(HashMap::new());
        entry2.insert("age".to_string(), "30".to_string());
    }
    //let mut cache_data = cache2.borrow_mut();
    //let entry2 = cache_data.storage.entry("user".to_string()).or_insert(HashMap::new());
    //entry2.insert("age".to_string(), "30".to_string());
    //cache2.borrow_mut().set("user".to_string(), data1.clone());

    println!("{:?}", cache.borrow().get(&"user".to_string()));
    println!("{}", cache.borrow().has(&"user".to_string()));
    println!("{}", cache.borrow().has(&"nonexistent".to_string()));

    let mut number_stack = NumberStack::new();
    number_stack.push(10.0);
    number_stack.push(5.5);
    number_stack.push(3.2);
    println!("{}", number_stack.sum());

    let mut list = LinkedList::new();
    list.append(10);
    list.append(20);
    list.append(30);
    list.print();

    let mut tree = BinaryTree::new();
    tree.insert(10);
    tree.insert(5);
    tree.insert(15);
    tree.insert(3);
    tree.insert(7);
    tree.print_in_order();

    let queue = Arc::new(Mutex::new(SharedQueue::new()));

    let mut threads = Vec::new();

    for i in 0..5 {
        let multi_queue_clone = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            let mut shared = multi_queue_clone.lock().unwrap();
            shared.enqueue(i);
        });
        threads.push(handle);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let mut result = queue.lock().unwrap();
    println!("Get Queue Params: {}", result.dequeue().unwrap());
    println!("Get Queue Params: {}", result.dequeue().unwrap());
    println!("Final Queue Size: {}", result.size());

    let counter_ex = Arc::new(Mutex::new(CounterEx::new()));
    let mut threads = Vec::new();

    for _ in 0..10 {
        let multi_counter_ex_clone = Arc::clone(&counter_ex);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut shared = multi_counter_ex_clone.lock().unwrap();
                shared.increment();
            }
        });
        threads.push(handle);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let result = counter_ex.lock().unwrap();
    println!("Final Counter Value: {}", result.get());

    let stack = Arc::new(Mutex::new(ParallelStack::new()));
    let mut threads = Vec::new();

    for i in 0..10 {
        let stack_clone = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            let mut shared = stack_clone.lock().unwrap();
            shared.push(i);
        });
        threads.push(handle);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let mut result = stack.lock().unwrap();
    while !result.is_empty() {
        println!("Stack pop Value: {}", result.pop().unwrap());
    }
    //println!("Stack peek Value: {}", result.peek().unwrap());
    println!("Stack is_empty Value: {}", result.is_empty());

    let user = User::new(1, "Alice".to_string());
    user.display();

    let product = Product::new(101, "Laptop".to_string(), 999.99);
    product.display();

    let rectangle = Rectangle::new(10.5, 5.2);
    println!("Area: {}", rectangle.area());
    println!("Perimeter: {}", rectangle.perimeter());

    let point_3d = Point3D::new(3.0, 4.0, 5.0);
    println!("Distance from origin: {}", point_3d.distance_from_origin());

    let circle = DimensionCircle::new(5.0);
    let rectangle = DimensionRectangle::new(10.0, 4.0);
    println!("Circle Area: {}", circle.area());
    println!("Recttangle Area: {}", rectangle.area());

    let console_logger = ConsoleLogger::new();
    let file_logger = FileLogger::new();
    console_logger.log("Hello, Rust Logger!".to_string());
    file_logger.log("Hello, Rust Logger!".to_string());

    let credit_card = CreditCardPayment::new();
    let paypal = PaypalPayment::new();

    credit_card.process_payment(100.50);
    paypal.process_payment(100.50);

    let car_travel = Travel::new(TransportMode::Car, 50.0);
    println!("Car travel time: {} hours", car_travel.calcuate_time());
    let bicycle_travel = Travel::new(TransportMode::Bicycle, 50.0);
    println!("Bicycle travel time: {} hours", bicycle_travel.calcuate_time());
    let walking_travel = Travel::new(TransportMode::Walking, 10.0);
    println!("Walking travel time: {} hours", walking_travel.calcuate_time());

    let payment1 = PaymentEx::new(PaymentMethod::CreditCard, 100.0, "4111111111".to_string());
    payment1.process_payment();
    let payment2 = PaymentEx::new(PaymentMethod::Paypal, 50.0, "user@example.com".to_string());
    payment2.process_payment();
    let payment3 = PaymentEx::new(PaymentMethod::Bitcoin, 0.05, "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string());
    payment3.process_payment();
}

use std::fmt::Debug;
use std::fmt::Display;
use std::cmp::PartialOrd;

#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}

fn main() {
    let item = return_item(5);
    print_item(5);

    let charlie = Animal {
        name: "Charlie".to_string(),
        age: 1,
    };
    let number = 55;
    print_item(charlie);
    print_item(number);

    compare_and_display("Listen up!", 9, 8);
    compare_and_display2("Listen up!", 9, 8);

    say_two("Hello there!", String::from("I hate sand."));
    say_two(String::from("Where is Padme?"), String::from("Is she all right?"));
}

fn say_two<T: Display, U: Display>(statement_1: T, statement_2: U) {
    println!("I have two things to say: {statement_1} and {statement_2}");
}

fn compare_and_display2<T, U>(statement: T, num_1: U, num_2: U)
    where T: Display, U: Display + PartialOrd,
{
    println!("{statement}! Is {num_1} greater than {num_2}? {}", num_1 > num_2);
}

fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, input_1: U, input_2: U) {
    println!("{statement}! Is {input_1} greater than {input_2}? {}", input_1 > input_2);
}

fn print_item<T: Debug>(item: T) {
    println!("Here is your item: {item:?}");
}

// fn print_item<T>(item: T) {
//     println!("Here is your item: {item:?}");
// }
fn return_item<MyType>(item: MyType) -> MyType {
    println!("Here is your item.");
    item
}

// fn return_item<T>(item: T) -> T {
//     println!("Here is your item.");
//     item
// }

// fn return_item(item: i32) -> i32 {
//     println!("Here is your item.");
//     item
// }
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

    let new_vec = vec![1, 2];
    //let index = take_fifth_item(new_vec);// this throws runtime error panic

    let small = vec![1, 2];
    let big = vec![1, 2, 3, 4, 5];
    println!("{:?}, {:?}", try_take_fifth(small), try_take_fifth(big));

    let small2 = vec![1, 2];
    let big2 = vec![1, 2, 3, 4, 5];

    // println!("{:?}, {:?}",
    //     try_take_fifth(small2).unwrap(), //same panic here
    //     try_take_fifth(big2).unwrap()
    // );
    let mut option_vec = Vec::new();
    option_vec.push(try_take_fifth(small2));
    option_vec.push(try_take_fifth(big2));
    handle_options(&option_vec);

    let small = vec![1, 2];
    let big = vec![1, 2, 3, 4, 5];
    for vec in vec![small, big] {
        let inside_number = try_take_fifth(vec);
        if inside_number.is_some() {
            println!("We got: {}", inside_number.unwrap());
        } else {
            println!("We got nothing.");
        }
    }

    check_error();

    if see_if_number_is_even(5).is_ok() {
        println!("It's okay, guys")
        } else {
        println!("It's an error, guys")
    }

    for number in 4..=7 {
        println!("{:?}", check_if_five(number));
    }

    let ok_value: Result<i32, &str> = Ok(7);
    let error_value: Result<i32, &str> = Err("There was an error");
    let unwrapped = ok_value.unwrap();
    //error_value.unwrap();//panic here
}

fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err(format!("Sorry, bad number. Expected: 5 Got: {number}")),
    }
}

fn see_if_number_is_even(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        return Ok(())
    } else {
        return Err(())
    }
}
fn check_error() -> Result<(), ()> {
    Ok(())
}
fn handle_options(my_option: &Vec<Option<i32>>) {
    for item in my_option {
        match item {
            Some(number) => println!("Found a {number}!"),
            None => println!("Found a None!"),
        }
    }
}

fn try_take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn take_fifth_item(value: Vec<i32>) -> i32 {
    value[4]
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
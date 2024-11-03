#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Display;
use std::mem::size_of;
use std::error::Error;
use std::fmt;
use std::sync::mpsc::RecvError;

fn main() {
    let char1 = 'ん';
    let char2 = ';';
    let some_str = "I'm just a regular &str";
    let some_vec = vec!["I", "am", "just", "a", "vec"];

    let my_string = HoldsAString {
        the_string: "Here I am!".to_string(),
    };
    println!("{:?}", my_string);

    let number_and_bool = NumberAndBool {
        number: 8,
        true_or_false: true
    };
    does_nothing(number_and_bool);
    does_nothing(number_and_bool);

    deprecated_function();

    let my_number = 1;
    just_takes_a_variable(my_number);
    just_takes_a_variable(my_number);
    let my_box = Box::new(1);
    just_takes_a_variable(my_box.clone());
    just_takes_a_variable(my_box);

    let my_box = Box::new(1);
    let an_integer: i32 = *my_box;

    let x = Holder {
        next_holder: Some(Box::new(Holder {
            next_holder: Some(Box::new(Holder { next_holder: None })),
        })),
    };

    println!("{x:#?}");

    println!(
        "{}, {}, {}, {}, {}",
        size_of::<EnumOfNumbers>(),
        size_of::<StructOfNumbers>(),
        size_of::<EnumOfOtherTypes>(),
        size_of::<StructOfOtherTypes>(),
        size_of::<ArrayAndI8>(),
    );

    let vec_of_u8s = vec![0_u8, 1, 80];
    for number in vec_of_u8s {
        match returns_errors(number) {
            Ok(input) => println!("{}", input),
            Err(message) => println!("{}", message),
        }
    }

    //let my_number = parse_numbers("8", "ninepointnine").unwrap();
    let err = MyError::TooMuchStuff;
    println!("{err}");
    println!("{err:?}");

    let errs = [true, false, false, true]
        .into_iter()
        .map(|boolean| give_error_back(boolean))
        .collect::<Vec<_>>();
    println!("{errs:#?}");
    for err in errs.iter() {
        if let Some(my_error) = err.downcast_ref::<MyError>() {
            println!("Got a MyError!");
        } else if let Some(parse_error) = err.downcast_ref::<RecvError>() {
            println!("Got a RecvError!");
        }
    }
}

fn give_error_back(is_true: bool) -> Box<dyn Error> {
    if is_true {
        Box::new(MyError::TooMuchStuff)
    } else {
        Box::new(RecvError)
    }
}

enum MyError {
    TooMuchStuff,
    CantConnect,
    NoUserRegistered,
    SomethingElse,
}

impl std::error::Error for MyError {}
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Wouldn't you like to know...")
    }
}
impl fmt::Debug for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lol not telling you what went wrong").finish()
    }
}

fn parse_numbers(int: &str, float: &str) -> Result<f64, Box<dyn Error>> {
    let num_1 = int.parse::<i32>()?;
    let num_2 = float.parse::<f64>()?;
    Ok(num_1 as f64 + num_2)
}

#[derive(Debug)]
struct ErrorOne;
impl Error for ErrorOne {}
impl fmt::Display for ErrorOne {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got the first error!")
    }
}

#[derive(Debug)]
struct ErrorTwo;
impl Error for ErrorTwo {}
impl fmt::Display for ErrorTwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got the second error!")
    }
}
fn returns_errors(input: u8) -> Result<String, Box<dyn Error>> {
    match input {
        0 => Err(Box::new(ErrorOne)),
        1 => Err(Box::new(ErrorTwo)),
        _ => Ok("Looks fine to me".to_string()),
    }
}

fn returns_just_a_trait() -> Box<dyn JustATrait> {
    let some_enum = EnumOfNumbers::I8(8);
    Box::new(some_enum)
}
// fn returns_just_a_trait() -> JustATrait {
//     let some_enum = EnumOfNumbers::I8(8);
//     some_enum
// }

trait JustATrait {}

enum EnumOfNumbers {
    I8(i8),
    AnotherI8(i8),
    OneMoreI8(i8),
}
impl JustATrait for EnumOfNumbers {}

struct StructOfNumbers {
    an_i8: i8,
    another_i8: i8,
    one_more_i8: i8,
}
impl JustATrait for StructOfNumbers {}
enum EnumOfOtherTypes {
    I8(i8),
    AnotherI8(i8),
    Collection(Vec<String>),
}
impl JustATrait for EnumOfOtherTypes {}

struct StructOfOtherTypes {
    an_i8: i8,
    another_i8: i8,
    a_collection: Vec<String>,
}
impl JustATrait for StructOfOtherTypes {}
struct ArrayAndI8 {
    array: [i8; 1000],
    an_i8: i8,
    in_u8: u8,
}
impl JustATrait for ArrayAndI8 {}

struct DoesntImplementDisplay {}
fn displays_it<T: Display>(input: T) {
    println!("{}", input);
}

#[derive(Debug)]
struct Holder {
    next_holder: Option<Box<Holder>>,
}

fn just_takes_a_variable<T>(item: T) {}

#[deprecated]
fn deprecated_function() {}

fn does_nothing(input: NumberAndBool) {}

#[derive(Clone, Copy)]
struct NumberAndBool {
    number: i32,
    true_or_false: bool
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Clone)]
struct HoldsAString {
    the_string: String,
}

struct Struct1 {}
struct Struct2 {}
struct Struct3 {}
struct Struct4 {}
struct Struct5 {}
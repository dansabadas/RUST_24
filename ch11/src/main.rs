use std::cell::{Cell, RefCell};
type SkipFourTakeFive =
    std::iter::Take<std::iter::Skip<std::vec::IntoIter<char>>>;
use std::iter::{Take, Skip};
use std::vec::IntoIter;
use std::borrow::Cow;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");

    use String as S;
    let my_string = S::from("Hi!");

    let book_type = BookType::HardCover;
    // Okay, let's check this function!
    check_book_type(&book_type);

    let msg1 = generate_message(
        "Everything is fine",
        None
        );
    let msg2 = generate_message(
        "Got an error",
        Some(ErrorInfo {
        error: LocalError::TooBig,
        message: "It was too big".to_string(),
        }),
    );

    for msg in [msg1, msg2] {
        match msg {
            Cow::Borrowed(msg) => {
                println!("Borrowed, didn't need an allocation:\n {msg}")
            }
            Cow::Owned(msg) => {
                println!("Owned, because we needed an allocation:\n {msg}")
            }
        }
    }

    let user_name = "User1";
    let other_user_name = "User10".to_string();
    let user1 = User {
        name: user_name.into(),
    };
    let user2 = User {
        name: other_user_name.into(),
    };
    for name in [user1.name, user2.name] {
        match name {
            Cow::Borrowed(n) => {
                println!("Borrowed name, didn't need an allocation:\n {n}")
            }
            Cow::Owned(n) => {
                println!("Owned name because we needed an allocation:\n {n}")
            }
        }
    }

    let user_name = "User1";
    let other_user_name = &"User10".to_string();
    let user1 = User2 {
        name: user_name.into(),
    };

    let user2 = User2 {
        name: other_user_name.into(),
    };
    for name in [user1.name, user2.name] {
        match name {
            Cow::Borrowed(n) => {
                println!("Borrowed name, didn't need an allocation:\n {n}")
            }
            Cow::Owned(n) => {
                println!("Owned name because we needed an allocation:\n {n}")
            }
        }
    }

    // let calgary = City {
    //     name: "Calgary".to_string(),
    //     population: 1_200_000,
    //     city_history: "Calgary began as a fort called Fort Calgary that...".to_string(),
    // };
    // let canada_cities = CityData {
    //     names: vec![calgary.name],
    //     histories: vec![calgary.city_history],
    // };

    // //println!("Calgary's history is: {}", calgary.city_history);
    // println!("Calgary's history is: {}", canada_cities.histories[0]);

    let calgary_name = Rc::new("Calgary".to_string());
    let calgary_history = Rc::new("Calgary began as a fort called Fort Calgary that...".to_string());
    let calgary = City {
        name: Rc::clone(&calgary_name),
        population: 1_200_000,
        city_history: Rc::clone(&calgary_history)
    };
    let canada_cities = CityData {
        names: vec![Rc::clone(&calgary_name)],
        histories: vec![Rc::clone(&calgary_history)],
    };
    println!("Calgary's history is: {}", calgary.city_history);
    println!("{}", Rc::strong_count(&calgary.city_history));

    let user_name = Rc::new(String::from("User MacUserson"));
    takes_a_string2(Rc::clone(&user_name));
    takes_a_string2(Rc::clone(&user_name));

    let city_names = vec![
        Rc::new("Ichinomiya".to_string()),
        Rc::new("Kurume".to_string()),
    ];
    let my_city = City3 {
        name: Rc::clone(&city_names[0]),
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded);
    std::thread::spawn(|| {
        println!("I am printing something");
    });

    let mut join_handles = vec![];
    for num in 0..10 {
        let handle = std::thread::spawn(move || {
            //println!("I am printing something");
            println!("Inside thread number: {num}");
        });
        //handle.join();
        join_handles.push(handle);
    }

    for handle in join_handles {
        handle.join().unwrap();
    }

    let my_string = String::from("I will go into the closure");
    let my_closure = || println!("{my_string}");
    my_closure();
    my_closure();

    let mut my_string = String::from("I will be changed in the closure");
    let mut my_closure = || {
        my_string.push_str(" now");
        println!("{my_string}");
    };
    my_closure();
    my_closure();

    let my_vec: Vec<i32> = vec![8, 9, 10];
    let my_closure = || {
        my_vec.into_iter().for_each(|item| println!("{item}"));
    };
    my_closure();
    // my_closure();

    let my_string = String::from("Can I go inside the thread?");
    let handle = std::thread::spawn(move || {
        println!("{my_string}");
    });
    handle.join().unwrap();
}

#[derive(Debug)]
struct City3 {
    name: Rc<String>,
    date_founded: u32,
}
#[derive(Debug)]
struct Country3 {
    cities: Vec<City3>,
}
#[derive(Debug)]
struct World3 {
    countries: Vec<Country3>,
}

impl World3 {}

#[derive(Debug)]
struct City2<'a> {
    name: &'a str,
    date_founded: u32,
}
#[derive(Debug)]
struct Country2<'a> {
    cities: Vec<City2<'a>>
}

#[derive(Debug)]
struct World2<'a> {
    countries: Vec<Country2<'a>>
}

fn takes_a_string2(input: Rc<String>) {
    println!("It is: {input}")
}

#[derive(Debug)]
struct CityData {
    names: Vec<Rc<String>>,
    histories: Vec<Rc<String>>,
}

#[derive(Debug)]
struct City {
    name: Rc<String>,
    population: u32,
    city_history: Rc<String>,
}

struct User2<'a> {
    name: Cow<'a, str>,
}
struct User {
    name: Cow<'static, str>,
}
#[derive(Debug)]
struct ErrorInfo {
    error: LocalError,
    message: String,
}
#[derive(Debug)]
enum LocalError {
    TooBig,
    TooSmall,
}

fn generate_message(
    message: &'static str,
    error_info: Option<ErrorInfo>
) -> Cow<'static, str> {
    match error_info {
        None => message.into(),
        Some(info) => format!("{message}: {info:?}").into(),
    }
}

fn returns_some_chars2(input: Vec<char>) -> Take<Skip<IntoIter<char>>> {
    input.into_iter().skip(4).take(5)
}
fn returns_some_chars(input: Vec<char>) -> SkipFourTakeFive {
    input.into_iter().skip(4).take(5)
}
// Okay, first I need a book struct.
// Nothing in there yet - will add later
struct Book;
// A book can be hardcover or softcover, so add an enum…
enum BookType {
    HardCover,
    SoftCover,
}
// should take a &Book and return an Option<String>
fn get_book(book: &Book) -> Option<String> {
    todo!();
}
// should take a ref Book and return a Result...
fn delete_book(book: &Book) -> Result<(), String> {
    todo!();
}
// TODO: impl block and make these functions methods…
// TODO: make this a proper error
fn check_book_type(book_type: &BookType) {
    // Let's make sure the match statement works
    match book_type {
        BookType::HardCover => println!("It's hardcover"),
        BookType::SoftCover => println!("It's softcover"),
    }
}
enum FileState {
    CannotAccessFile,
    FileOpenedAndReady,
    NoSuchFileExists,
    SimilarFileNameInNextDirectory,
}

fn give_filestate(input: &FileState) {
    use FileState::{
    CannotAccessFile as NoAccess,
    FileOpenedAndReady as 잘됨,
    NoSuchFileExists as NoFile,
    SimilarFileNameInNextDirectory as OtherDirectory
    };
    match input {
        NoAccess => println!("Can't access file."),
        잘됨 => println!("Here is your file"),
        NoFile => println!("Sorry, there is no file by that name."),
        OtherDirectory => println!("Please check the other directory."),
    }
}
enum MapDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

// fn give_direction(direction: &MapDirection) {
//     //use MapDirection::*;
//     // match direction {
//     //     North => println!("You are heading north."),
//     //     NorthEast => println!("You are heading northeast."),
//     // }
// }
use rand::seq::SliceRandom;
use std::fmt::Display;
use std::io;

fn main() {
    let my_letters = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];
    let mut rng = rand::thread_rng();
    for _ in 0..6 {
        print!("{} ", my_letters.choose(&mut rng).unwrap());
    }

    let my_name = print_and_return("Windy");
    let small_number = print_and_return(9.0);

    println!("Please type something, or x to escape:");
    let mut input_string = String::new();

    while input_string.trim() != "x" {
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();
        println!("You wrote {input_string}");
    }
    println!("See you later!");

    println!("{:?}", std::env::args());
}

pub fn add() -> i32 {
    let mut sum = 0;
    for _ in 0..1000 {
        sum += 1
    }

    sum
}

fn print_and_return<T: Display>(input: T) -> T {
    println!("You gave me {input} and now I will give it back.");
    input
}

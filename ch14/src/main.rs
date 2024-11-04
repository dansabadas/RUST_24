fn main() {
    use print_things::prints_one_thing;

    prints_one_thing(6);
    prints_one_thing("Trying to print a string...".to_string());

    use print_things::*;
    let my_billy = Billy::new(3);
    my_billy.print_billy();

    country::province::city::print_city("Canada", "New Brunswick", "Moncton");
}

const OKAY_CHARACTERS: &str = "1234567890- ";

#[derive(Default)]
struct Subtractor{
    total: i32,
    num_to_parse: String,
    operation: Operation,
}

#[derive(Default)]
enum Operation {
    #[default]
    Add,
    Subtract,
}

impl Subtractor {
    fn switch_operation(&mut self) {
        self.operation = match self.operation {
            Operation::Add => Operation::Subtract,
            Operation::Subtract => Operation::Add,
        }
    }

    fn do_operation(&mut self) {
        let num = self.num_to_parse.parse::<i32>().unwrap();
        match self.operation {
            Operation::Add => self.total += num,
            Operation::Subtract => self.total -= num,
        }
        self.operation = Operation::Add;
        self.num_to_parse.clear();
    }

    fn math(&mut self, input: &str) -> i32 {
        if input
        .chars()
        .any(|character| !OKAY_CHARACTERS.contains(character))
        {
            panic!("Please only input numbers, -, or spaces.");
        }
        let input = input
            .trim_end_matches(|x| "- ".contains(x))
            .chars()
            .filter(|x| *x != ' ')
            .collect::<String>();
        for character in input.chars() {
            match character {
                '-' => {
                    if !self.num_to_parse.is_empty() {
                        self.do_operation();
                    }
                    self.switch_operation();
                }
                number => self.num_to_parse.push(number),
            }
        }
        if !self.num_to_parse.is_empty() {
            self.do_operation();
        }
        self.total
    }
}

#[cfg(test)]
mod tests2
{
    use super::*;
    #[test]
    fn one_minus_two_is_minus_one() {
        let mut calc = Subtractor::default();
        assert_eq!(calc.math("1 - 2"), -1);
    }

    #[test]
    fn one_minus_minus_one_is_two() {
        let mut calc = Subtractor::default();
        assert_eq!(calc.math("1 - -1"), 2);
    }

    #[test]
    fn three_minus_three_minus_three_minus_minus_three_is_zero() {
        let mut calc = Subtractor::default();
        assert_eq!(calc.math("3-3-3--3"), 0);
    }

    #[test]
    fn eighteen_minus_nine_minus_nine_is_zero_even_with_characters_on_the_end() {
        let mut calc = Subtractor::default();
        assert_eq!(calc.math("18 - 9 -9-----"), 0);
    }

    #[test]
    #[should_panic]
    fn panics_when_characters_not_right() {
        let mut calc = Subtractor::default();
        calc.math("7 - seven");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_returns_six() {
        assert_eq!(return_six(), 6)
    }

    #[test]
    fn it_returns_two() {
        assert_eq!(return_two(), 2);
    }
}

fn return_two() -> i8 {
    2
}
#[test]
fn it_returns_two() {
    assert_eq!(return_two(), 2);
}
fn return_six() -> i8 {
    4 + return_two()
}
#[test]
fn it_returns_six() {
    assert_eq!(return_six(), 6)
}

#[test]
fn two_is_two() {
    assert_eq!(2, 2);
}

mod country {
    fn print_country(country: &str) {
        println!("We are in the country of {country}");
    }
    pub mod province {
        fn print_province(province: &str) {
            println!("in the province of {province}");
        }
        pub mod city {
            pub fn print_city(country: &str, province: &str, city: &str) {
                crate::country::print_country(country);
                super::super::print_country(country);
                crate::country::province::print_province(province);
                super::print_province(province);
                println!("in the city of {city}");
            }
        }
    }
}
mod print_things {
    use std::fmt::Display;

    #[derive(Debug)]
    pub struct Billy {
        name: String,
        pub times_to_print: u32,
    }

    impl Billy {
        pub fn new(times_to_print: u32) -> Self {
            Self {
                name: "Billy".to_string(),
                times_to_print,
        }
        }

        pub fn print_billy(&self) {
            for _ in 0..self.times_to_print {
                println!("{}", self.name);
            }
        }
    }

    pub fn prints_one_thing<T: Display>(input: T) {
        println!("{input}");
    }
}
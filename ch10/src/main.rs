use std::fmt::Display;


fn prints_str(my_str: &str) {
    println!("{my_str}");
}

fn works() -> &'static str {
    "I live forever!"
}

fn returns_str() -> &'static str {
    let my_string = String::from("I am a string");
    "I am a str"
}

fn main() {
    let my_string = String::from("I am a string");
    prints_str(&my_string);
    //prints_str(my_string);

    let my_str = returns_str();
    println!("{my_str}");

    let my_city = City {
        name: "Ichinomiya",
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded);

    let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];
    let my_city = City2 {
        name: &city_names[0],
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded);

    let mut billy = Adventurer {
        name: "Billy",
        hit_points: 100_000,
    };
    println!("{}", billy);
    billy.take_damage();
}

struct Adventurer<'a> {
    name: &'a str,
    hit_points: u32,
}

impl Adventurer<'_> {
    fn take_damage(&mut self) {
        self.hit_points -= 20;
        println!("{} has {} hit points left!", self.name, self.hit_points);
    }
}

impl std::fmt::Display for Adventurer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} has {} hit points.", self.name, self.hit_points)
    }
}

fn prints<T: Display>(input: T) {
    println!("T is {input}");
}

#[derive(Debug)]
struct City3<'city> {
    name: &'city str,
    date_founded: u32,
}

#[derive(Debug)]
struct City2<'a> {
    name: &'a str,
    date_founded: u32,
}

#[derive(Debug)]
struct City {
    name: &'static str,
    date_founded: u32,
}
use std::fmt;

struct Dog {
    name: String,
}

struct Parrot {
    name: String,
}

trait DogLike {
    fn bark(&self) {
        println!("Woof woof!");
    }
    fn run(&self) {
        println!("The dog is running!");
    }
}

impl DogLike for Dog {}
//impl DogLike for Parrot {}
impl DogLike for Parrot{
    fn run(&self) {
        println!("{} the parrot is running!", self.name);
    }
}

struct Animal {
    name: String,
}

trait DogLike2 {
    fn bark(&self);
    fn run(&self);
}

impl DogLike2 for Animal {
    fn bark(&self) {
        println!("{}, stop barking!!", self.name);
    }
    fn run(&self) {
        println!("{} is running!", self.name);
    }
}

#[derive(Debug)]
struct Cat {
    name: String,
    age: u8,
}

struct Position {
    longitude: f32,
    latitude: f32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.longitude, self.latitude)
    }
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is a cat who is {} years old", self.name, self.age)
    }
}

fn print_excitedly(input: String) {
    println!("{input}!!!!!");
}

fn main() {
    let rover = Dog {
        name: "Rover".to_string(),
    };
    let brian = Parrot {
        name: "Brian".to_string(),
    };
    rover.bark();
    rover.run();
    brian.bark();
    brian.run();

    let rover = Animal {
        name: "Rover".to_string(),
    };
    rover.bark();
    rover.run();

    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };

    println!("Mr. Mantle is a {mr_mantle:?}");
    println!("Mr. Mantle is a {mr_mantle}");

    print_excitedly(mr_mantle.to_string());
    println!(
        "Mr. Mantle's String is {} letters long.",
        mr_mantle.to_string().chars().count()
    );
}

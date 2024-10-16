struct FileDirectory;
struct ColorRgb(u8, u8, u8);

struct SizeAndColor {
    size: u32,
    color: ColorRgb,
}
enum Climate {
    Tropical,
    Dry,
    Temperate,
    Continental,
    Polar,
}
struct Country {
    population: u32,
    capital: String,
    leader_name: String,
    climate: Climate,
}
enum ThingsInTheSky {
    Sun,
    Stars,
}
enum ThingsInTheSkyWithDiamonds {
    Sun(String),
    Stars(String),
}

enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry,
}

use std::mem::size_of_val;

enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar,
}
enum Number {
    U32(u32),
    I32(i32),
}
#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}
#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}
impl Animal {
    fn new_cat() -> Self {
        Self {
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }
    fn check_type(&self) {
        match self.animal_type {
            AnimalType::Dog => println!("The animal is a dog"),
            AnimalType::Cat => println!("The animal is a cat"),
        }
    }
    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
        println!("Changed animal to dog! Now it's {self:?}");
    }
    fn change_to_cat(&mut self) {
        self.animal_type = AnimalType::Cat;
        println!("Changed animal to cat! Now it's {self:?}");
    }
}
enum Mood2 {
    Good,
    Bad,
    Sleepy,
}
impl Mood2 {
    fn check(&self) {
        match self {
            Mood2::Good => println!("Feeling good!"),
            Mood2::Bad => println!("Eh, not feeling so good"),
            Mood2::Sleepy => println!("Need sleep NOW"),
        }
    }
}
fn main() {
    let my_color = ColorRgb(50, 0, 50);
    println!("The second part of the color is: {}", my_color.1);
    let size_and_color = SizeAndColor {
        size: 150,
        color: my_color
    };

    let population = 500_000;
    let capital = String::from("Elista");
    let leader_name = String::from("Batu Khasikov");
    // let kalmykia = Country {
    //     population: population,
    //     capital: capital,
    //     leader_name: leader_name,
    // };
    let kalmykia = Country {
        population,
        capital,
        leader_name,
        climate: Climate::Continental,
    };
    let kalmykia = Country {
        population: 500_000,
        capital: String::from("Elista"),
        leader_name: String::from("Batu Khasikov"),
        climate: Climate::Dry
    };

    let time = 8;
    let skystate = create_skystate(time);
    check_skystate(&skystate);
    let time = 8;
    let skystate = create_skystate2(time);
    check_skystate2(&skystate);

    let my_mood = Mood::Happy;
    let happiness_level = match_mood(&my_mood);
    println!("Out of 1 to 10, my happiness is {happiness_level}");

    let size_of_jaurim = size_of_val("자우림");
    let size_of_adrian = size_of_val("Adrian Fahrenheit ?epe?");
    println!("{size_of_jaurim}, {size_of_adrian}");

    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumn, Winter];
    for season in four_seasons {
        println!("{}", season as u32);
    }
    use Star::*;
    let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant, DeadStar];
    for star in starvec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star."),
            size if size >= 80 && size <= 200 => println!("This is a good-sized star."),
            other_size => println!("That star is pretty big! It's {other_size}"),
        }
    }
    let my_vec = vec![get_number(-800), get_number(8)];
    for item in my_vec {
        match item {
            //Number::U32 => println!("A u32  value"),
            Number::U32(number) => println!("A u32 with the value {number}"),
            Number::I32(number) => println!("An i32 with the value {number}"),
        }
    }

    let mut my_string = String::from("I feel excited");
    my_string.push('!');

    let mut new_animal = Animal::new_cat();
    new_animal.check_type();
    new_animal.change_to_dog();
    new_animal.check_type();
    new_animal.change_to_cat();
    new_animal.check_type();

    let my_mood = Mood2::Sleepy;
    my_mood.check();

    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false
    };
    // let Person {
    //     name,
    //     real_name,
    //     height,
    //     happiness,
    // } = papa_doc;
    let Person {
        name: fake_name,
        real_name,
        height: cm,
        happiness
        } = papa_doc;

    // println!("They call him {name} but his real name is {real_name}.
    // He is {height} cm tall and is he happy? {happiness}");
    println!("They call him {fake_name} but his real name is {real_name}. He is {cm} cm tall and is he happy? {happiness}");

    let tallinn = City::new("Tallinn", "Reval", 426_538, 1219);
    tallinn.print_names();

    let papa_doc3 = Person3 {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };
    check_if_happy(&papa_doc3);
    check_if_happy_destructured(&papa_doc3);

    let my_number = 9;
    let reference = &my_number;

    println!("{}", my_number == *reference);

    let my_name = "Billy".to_string();
    let double_ref = &&my_name;
    println!("{}", my_name);
    println!("{}", double_ref.is_empty());
    println!("{}", (&**double_ref).is_empty());
    println!("{}", (**double_ref));
    println!("{}", &&&&&double_ref.is_empty());
    println!("{}", &&&&&double_ref);
}
struct City {
    name: String,
    name_before: String,
    population: u32,
    date_founded: u32,
}
impl City {
    fn new(
        name: &str,
        name_before: &str,
        population: u32,
        date_founded: u32,
    ) -> Self {
        Self {
            name: String::from(name),
            name_before: String::from(name_before),
            population,
            date_founded,
        }
    }
    fn print_names(&self) {
        let City {
            name,
            name_before,
            ..
            // population,
            // date_founded,
        } = self;
        println!("The city {name} used to be called {name_before}.");
    }
}
struct Person3 {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}
fn check_if_happy(person: &Person3) {
    println!("Is {} happy? {}", person.name, person.happiness);
}
fn check_if_happy_destructured(Person3 { name, happiness, .. }: &Person3) {
    println!("Is {name} happy? {happiness}");
}
struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool
}

fn get_number(input: i32) -> Number {
    let number = match input.is_positive() {
        true => Number::U32(input as u32),
        false => Number::I32(input),
    };
    number
}

fn match_mood(mood: &Mood) -> i32 {
    use Mood::*;
    let happiness_level = match mood {
        Happy => 10,
        Sleepy => 6,
        NotBad => 7,
        Angry => 2,
    };
    happiness_level
}

fn create_skystate2(time: i32) -> ThingsInTheSkyWithDiamonds {
    match time {
        6..=18 => ThingsInTheSkyWithDiamonds::Sun(String::from("I can see the sun!")),
        _ => ThingsInTheSkyWithDiamonds::Stars(String::from("I can see the stars!")),
    }
}
fn check_skystate2(state: &ThingsInTheSkyWithDiamonds) {
    match state {
        ThingsInTheSkyWithDiamonds::Sun(description) => println!("{description}"),
        ThingsInTheSkyWithDiamonds::Stars(n) => println!("{n}"),
    }
}
fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Stars,
    }
}
fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun!"),
        ThingsInTheSky::Stars => println!("I can see the stars!")
    }
}
//cargo new hello_world
//cargo build
//cargo run
//https://code.visualstudio.com/docs/languages/rust

//https://rustwasm.github.io/docs/book/introduction.html

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

use rand::{random, thread_rng, Rng};
use std::fmt;
use std::ops::Add;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let my_cities = ["Beirut", "Tel Aviv", "Nicosia"];
    for city in my_cities {
        println!("{}", city);
    }

    for city in &my_cities {
        println!("{city}");
    }
    for city in my_cities.iter() {
        println!("{city}");
    }

    let [city1, _city2, _city3] = my_cities;
    println!("{city1}");

    let [first, .., last] = my_cities;
    println!("{first}, {last}");

    let int_array = [1, 5, 9, 13, 17, 21, 25, 29];
    let string_array = int_array.map(|i| (i*2).to_string());
    println!("{int_array:?}");
    println!("{string_array:?}");

    let int_array = [1, 5, 9, 13, 17, 21, 25, 29];
    let hours_array = int_array.map(Hours::from);
    println!("{hours_array:?}");

    let array = std::array::from_fn(|i| i);
    assert_eq!(array, [0, 1, 2, 3, 4]);

    let array = std::array::from_fn(|_| "Don't care about the index");
    assert_eq!(array,
        [
            "Don't care about the index",
            "Don't care about the index",
            "Don't care about the index",
            "Don't care about the index",
            "Don't care about the index"
        ]
    );

    let array: [_; 5] = std::array::from_fn(|_| "Don't need the index");
    let array: [&str; 5] = std::array::from_fn(|_| "Don't need the index");

    let korean_word = "청춘예찬";
    for character in korean_word.chars() {
        print!("{} ", character.escape_unicode());
    }

    println!("This will always work: {}", char::from(100));
    println!("So will this: {}", char::from(random::<u8>()));
    for _ in 0..100_000 {
        if let Ok(successful_character) = char::try_from(random::<u32>()) {
            print!("{successful_character}");
        }
    }

    "Hi, привіт, 안녕, "
        .chars()
        .for_each(|c| println!("{c}: {}", c.len_utf8()));

    // let some_number = 200_u8;
    // println!("{}", some_number + 200);
    let mut rng = thread_rng();
    let some_number = 255_u8;
    //println!("{}", some_number + rng.gen_range(10..=10));

    for _ in 0..3 {
        let some_number = random::<u8>();
        let other_number = random::<u8>();
        add_numbers(some_number, other_number);
    }

    let nauru = Country::new("Nauru", 12_511, 133_200_000);
    let vanuatu = Country::new("Vanuatu", 219_137, 956_300_000);
    let micronesia = Country::new("Micronesia", 113_131, 404_000_000);
    println!("{}", nauru);
    let nauru_and_vanuatu = nauru + vanuatu;
    println!("{nauru_and_vanuatu}");
    println!("{}", nauru_and_vanuatu + micronesia);

    four_operations(9.1);
    four_operations(100.7);
    four_operations(-1.1);
    four_operations(-19.9);

    let mut my_struct = MyStruct("Hi".to_string());
    my_struct.print_self();
    MyStruct::print_self(&my_struct);
    my_struct.add_exclamation();
    MyStruct::add_exclamation(&mut my_struct);
    MyStruct::print_self(&my_struct);

    let num1 = 10;
    let num2 = 10;
    print!("{} ", i32::add(num1, num2));
    print!("{} ", num1.add(num2));
    print!("{}", num1 + num2);

    let string1 = "Hello there!".to_string();
    println!("{}", string1.change_form());
    let string2 = "I'm back!".to_string();
    println!("{}", String::change_form(string2));
    let small_num = 1;
    println!("{}", small_num.change_form());
    let also_small_num = 0;
    println!("{}", i32::change_form(also_small_num));

    println!("{:?}", SizeTenString::try_from("This one's long"));
    println!("{:?}", SizeTenString::try_from("Too short"));
    println!("{:?}", SizeTenString::try_from("Just right"));

    let nums = vec![8.0_f64, 7.6, 9.4, 10.0, 22.0, 77.345, -7.77, -10.0];
    let max = nums
        .iter()
        .fold(f64::MIN, |num, next_num| num.max(*next_num));
    let min = nums
        .iter()
        .fold(f64::MAX, |num, next_num| num.min(*next_num));
    println!("{max}, {min}");

    let true_false = (true, false);
    println!("{} {}", true_false.0 as u8, true_false.1 as i32);

    let true_false: (i128, u16) = (true.into(), false.into());
    println!("{} {}", true_false.0, true_false.1);

    let (tru, fals) = (true.then(|| 8), false.then(|| 8));
    println!("{:?}, {:?}", tru, fals);

    let bool_vec = vec![true, false, true, false, false];
    let result_vec = bool_vec
        .into_iter()
        .enumerate()
        .map(|(index, b)| {
            b.then(|| {
                let timestamp = timestamp();
                send_data_to_user();
                timestamp
            })
            .ok_or_else(|| {
                let time = timestamp();
                format!("Error with item {index} at {time}")
            })
        })
        .collect::<Vec<_>>();

    println!("{result_vec:#?}");

    let mut my_vec = vec![100, 90, 80, 0, 0, 0, 0, 0];
    my_vec.sort();
    println!("{:?}", my_vec);

    let mut my_vec = vec!["sun", "sun", "moon", "moon", "sun", "moon", "moon"];
    my_vec.sort();
    my_vec.dedup();
    println!("{:?}", my_vec);

    let mut big_vec = vec![0; 6];
    let (first, second) = big_vec.split_at_mut(3);
    std::thread::scope(|s| {
        s.spawn(|| {
            for num in first {
                *num += 1;
            }
        });
        s.spawn(|| {
            for num in second {
                *num -= 5;
            }
        });
    });
    println!("{big_vec:?}");

    let mut push_string = String::new();
    for _ in 0..100_000 {
        let capacity_before = push_string.capacity();
        push_string.push_str("I'm getting pushed into the string!");
        let capacity_after = push_string.capacity();
        if capacity_before != capacity_after {
            println!("Capacity raised to {capacity_after}");
        }
    }

    let mut push_string = String::with_capacity(4587520);
    for _ in 0..100_000 {
        let capacity_before = push_string.capacity();
        push_string.push_str("I'm getting pushed into the string!");
        let capacity_after = push_string.capacity();
        if capacity_before != capacity_after {
            println!("Capacity raised to {capacity_after}");
        }
    }
    println!("Current capacity as expected: {}", push_string.capacity());
    push_string.shrink_to_fit();
    println!("Actual needed capacity: {}", push_string.capacity());
    push_string.push('a');
    println!("Whoops, it doubled again: {}", push_string.capacity());
    push_string.shrink_to_fit();
    println!("Shrunk back to actual needed capacity: {}",
    push_string.capacity());

    let mut my_string = String::from(".daer ot drah tib elttil a si gnirts sihT");
    while let Some(c) = my_string.pop() {
        print!("{c}");
    }
}

fn timestamp() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

fn send_data_to_user() {}

#[derive(Debug)]
struct SizeTenString(String);
impl SizeTenString {
    const SIZE: usize = 10;
}

impl TryFrom<&'static str> for SizeTenString {
    type Error = String;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if input.chars().count() == Self::SIZE {
            Ok(Self(input.to_string()))
        } else {
            Err(format!("Length must be {} characters!", Self::SIZE))
        }
    }
}

trait ChangeForm {
    type SomethingElse;
    fn change_form(self) -> Self::SomethingElse;
}

impl ChangeForm for String {
    type SomethingElse = char;
    fn change_form(self) -> Self::SomethingElse {
        self.chars().next().unwrap_or(' ')
    }
}

impl ChangeForm for i32 {
    type SomethingElse = i64;
    fn change_form(self) -> Self::SomethingElse {
        println!("i32 just got really big!");
        i64::MAX
    }
}

struct MyStruct(String);
impl MyStruct {
fn print_self(&self) {
println!("{}", self.0);
}
fn add_exclamation(&mut self) {
self.0.push('!')
}
}

fn four_operations(input: f64) {
    println!(
        "For the number {}:
        floor: {}
        ceiling: {}
        rounded: {}
        truncated: {}\n",
        input,
        input.floor(),
        input.ceil(),
        input.round(),
        input.trunc()
    );
}

fn add_numbers(one: u8, two: u8) {
    match one.checked_add(two) {
        Some(num) => println!("Added {one} to {two}: {num}"),
        None => println!("Error: couldn't add {one} to {two}"),
    }
}

#[derive(Clone)]
struct Country {
    name: String,
    population: u32,
    gdp: u32,
}

impl Country {
    fn new(name: &str, population: u32, gdp: u32) -> Self {
        Self {
            name: name.to_string(),
            population,
            gdp,
        }
    }
}

impl Add for Country {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            name: format!("{} and {}", self.name, other.name),
            population: self.population + other.population,
            gdp: self.gdp + other.gdp,
        }
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "In {} are {} people and a GDP of ${}",
            self.name, self.population, self.gdp
        )
    }
}

#[derive(Debug)]
enum Hours {
    Working(u32),
    NotWorking(u32),
    Error(u32),
}

impl From<u32> for Hours {
    fn from(value: u32) -> Self {
        match value {
            hour if (8..17).contains(&hour) => Hours::Working(value),
            hour if (0..=24).contains(&hour) => Hours::NotWorking(value),
            wrong_hour => Hours::Error(wrong_hour),
        }
    }
}

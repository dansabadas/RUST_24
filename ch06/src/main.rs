use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::num::ParseIntError;

struct City {
    name: String,
    population: HashMap<i32, i32>,
}

struct City2 {
    name: String,
    population: BTreeMap<i32, i32>,
}

fn main() {
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: HashMap::new(),
    };

    tallinn.population.insert(2020, 437_619);
    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);

    for (year, population) in tallinn.population {
        println!("In {year}, Tallinn had a population of {population}.");
    }

    let mut tallinn2 = City2 {
        name: "Tallinn".to_string(),
        population: BTreeMap::new(),
    };

    tallinn2.population.insert(2020, 437_619);
    tallinn2.population.insert(1372, 3_250);
    tallinn2.population.insert(1851, 24_000);
    for (year, population) in tallinn2.population {
        println!("In {year}, Tallinn had a population of {population}.");
    }

    let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];

    let mut city_hashmap = HashMap::new();
    for city in canadian_cities {
        city_hashmap.insert(city, "Canada");
    }
    for city in german_cities {
        city_hashmap.insert(city, "Germany");
    }
    println!("{:?}", city_hashmap["Bielefeld"]);
    println!("{:?}", city_hashmap.get("Bielefeld"));
    println!("{:?}", city_hashmap.get("Bielefeldd"));
    println!("{:?}", city_hashmap.get("Bielefeld"));

    let mut book_hashmap = HashMap::new();
    book_hashmap.insert(1, "L'Allemagne Moderne");
    book_hashmap.insert(1, "Le Petit Prince");
    book_hashmap.insert(1, "섀도우 오브 유어 스마일");
    book_hashmap.insert(1, "Eye of the World");
    println!("{:?}", book_hashmap.get(&1));
    let key = 1;
    match book_hashmap.get(&key) {
        Some(val) => println!("Key {key} has a value already: {val}"),
        None => {
            book_hashmap.insert(key, "Le Petit Prince");
        }
    }
    println!("{:?}", book_hashmap.get(&1));

    let mut book_hashmap2 = HashMap::new();
    let mut old_hashmap_values = Vec::new();
    let hashmap_entries = [
        (1, "L'Allemagne Moderne"),
        (1, "Le Petit Prince"),
        (1, "섀도우 오브 유어 스마일"),
        (1, "Eye of the World"),
    ];

    for (key, value) in hashmap_entries {
        if let Some(old_value) = book_hashmap2.insert(key, value) {//insert returns old value being replaced
            println!("Overwriting {old_value} with {value}!");
            old_hashmap_values.push(old_value);
        }
    }

    println!("All old values: {old_hashmap_values:?}");

    let book_collection = vec![
        "L'Allemagne Moderne",
        "Le Petit Prince",
        "Eye of the World",
        "Eye of the World",
    ];
    let mut book_hashmap = HashMap::new();
    for book in book_collection {
        book_hashmap.entry(book).or_insert(true);
    }
    for (book, true_or_false) in book_hashmap {
        println!("Do we have {book}? {true_or_false}");
    }

    let book_collection = vec![
        "L'Allemagne Moderne",
        "Le Petit Prince",
        "Eye of the World",
        "Eye of the World",
    ];

    let mut book_hashmap3 = HashMap::new();
    for book in book_collection {
        let return_value = book_hashmap3.entry(book).or_insert(0);
        *return_value += 1;
    }
    for (book, number) in book_hashmap3 {
        println!("{book}, {number}");
    }

    let data = vec![
        ("male", 9),
        ("female", 5),
        ("male", 0),
        ("female", 6),
        ("female", 5),
        ("male", 10),
    ];
    let mut survey_hash = HashMap::new();
    for item in data {
        survey_hash.entry(item.0).or_insert(Vec::new()).push(item.1);
    }
    for (male_or_female, numbers) in survey_hash {
        println!("{male_or_female}: {numbers:?}");
    }
    let many_numbers = vec![
        37, 3, 25, 11, 27, 3, 37, 21, 36, 19, 37, 30, 48, 28, 16, 33, 2, 10, 1, 12, 38, 35, 30, 21,
        20, 38, 16, 48, 39, 31, 41, 32, 50, 7, 15, 1, 20, 3, 33, 12, 1, 11, 34, 38, 49, 1, 27, 9,
        46, 33,
    ];
    println!("How many numbers in the Vec? {}", many_numbers.len());

    let mut number_hashset = HashSet::new();
    for number in many_numbers {
        number_hashset.insert(number);
    }
    let hashset_length = number_hashset.len();
    println!(
        "There are {hashset_length} unique numbers, so we are missing {}.",
        50 - hashset_length
    );
    println!("It does not contain: ");
    for number in 0..=50 {
        if number_hashset.get(&number).is_none() {
            print!("{number} ");
        }
    }
    for entry in number_hashset {
        print!("{} ", entry);
    }

    let many_numbers = vec![37, 3, 25, 11, 27, 3, 37, 21, 36, 19, 37, 30, 48,
        28, 16, 33, 2, 10, 1, 12, 38, 35, 30, 21, 20, 38, 16, 48, 39, 31, 41,
        32, 50, 7, 15, 1, 20, 3, 33, 12, 1, 11, 34, 38, 49, 1, 27, 9, 46, 33];
    let mut current_number = i32::MIN;
    let mut number_set = BTreeSet::new();
    for number in many_numbers {
        number_set.insert(number);
    }
    for number in number_set {
        if number < current_number {
            println!("This will never happen");
        }
        current_number = number;
    }
    // for entry in number_set {
    //     print!("{} ", entry);
    // }

    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30];
    let mut heap = BinaryHeap::new();
    for num in many_numbers {
        heap.push(num);
    }
    println!("First item is largest, others are out of order: {heap:?}");
    while let Some(num) = heap.pop() {
        println!("Popped off {num}. Remaining numbers are: {heap:?}");
    }

    let mut jobs = BinaryHeap::new();
    jobs.push((100, "Reply to email from the CEO"));
    jobs.push((80, "Finish the report today"));
    jobs.push((5, "Watch some YouTube"));
    jobs.push((70, "Tell your team members thanks for always working hard"));
    jobs.push((30, "Plan who to hire next for the team"));
    for (_, job) in jobs {
        println!("You need to: {job}");
    }
    // while let Some(num) = jobs.pop() {
    //     println!("Popped off. Remaining numbers are: {heap:?}");
    // }

    let mut my_vec = VecDeque::from(vec![0; 600000]);
    for _ in 0..600000 {
        my_vec.pop_front();
        //my_vec.remove(0);
    }

    let str_vec = vec!["Seven", "8", "9.0", "nice", "6060"];
    for item in str_vec {
        let parsed = parse_and_log_str(item);
        println!("Result: {parsed:?}");
    }
    let str_vec = vec!["Seven", "8", "9.0", "nice", "6060"];
    for item in str_vec {
        let parsed = parse_str(item);
        println!("{parsed:?}");
    }

    let num = turn_into_string_and_parse(vec![49, 53, 53]);
    println!("{num}");

    let my_vec = vec![8, 9, 10];
    print_all_three_things(my_vec);

    let my_name = "Loki Laufeyson";
    assert!(my_name == "Loki Laufeyson");
    assert_eq!(my_name, "Loki Laufeyson");
    assert_ne!(my_name, "Mithridates");

    assert!(
        my_name == "Loki Laufeyson",
        "Name {my_name} is wrong: should be Loki Laufeyson"
    );
    assert_eq!(
        my_name, "Loki Laufeyson",
        "{my_name} and Loki Laufeyson should be equal"
    );
    assert_ne!(
        my_name, "Mithridates",
        "You entered {my_name}. Input must not equal Mithridates"
    );

    let my_vec = vec![9, 0, 10];
    //let fourth = get_fourth(&my_vec);

    let my_vec = vec![8, 9, 10];
    let fourth = my_vec.get(3).unwrap_or(&0);
    println!("{fourth}");
}

fn get_fourth(input: &Vec<i32>) -> i32 {
    //let fourth = input.get(3).unwrap();
    let fourth = input.get(3).expect("Input vector needs at least 4 items");
    *fourth
}

fn print_all_three_things(vector: Vec<i32>) {
    if vector.len() != 3 {
        panic!("my_vec must always have three items");
    }
    println!("{}, {}, {}", vector[0], vector[1], vector[2]);
}

fn turn_into_string_and_parse(bytes: Vec<u8>) -> i32 {
    let as_string = String::from_utf8(bytes).unwrap();
    let as_num = as_string.parse::<i32>().unwrap();
    as_num
}

fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input
        .parse::<u16>()?
        .to_string()
        .parse::<u32>()?
        .to_string()
        .parse::<i32>()?;
    println!("Number parsed successfully into {parsed_number}");
    Ok(parsed_number)
}

fn parse_and_log_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input.parse::<i32>()?;
    println!("Number parsed successfully into {parsed_number}");
    Ok(parsed_number)
}
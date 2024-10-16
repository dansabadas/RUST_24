use std::collections::HashMap;
use std::collections::BTreeMap;

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
}
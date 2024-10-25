fn main() {
    let months = vec!["January", "February", "March", "April", "May",
        "June", "July", "August", "September", "October", "November", "December"];
    let filtered_months = months
        .into_iter()
        .filter(|month| month.len() < 5)
        .filter(|month| month.contains("u"))
        .collect::<Vec<&str>>();
    println!("{:?}", filtered_months);

    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Brendan McCracken"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];
    let all_the_ceos = company_vec
        .iter()
        .filter_map(|company| company.get_ceo())
        .collect::<Vec<String>>();
    println!("{:?}", all_the_ceos);
    // let all_the_ceos2 = company_vec
    //     .iter()
    //     .filter_map(|company| company.get_ceo())
    //     .collect::<Vec<String>>();
    // println!("{:?}", all_the_ceos2);

    let user_input = vec![
        "8.9",
        "Nine point nine five",
        "8.0",
        "7.6",
        "eleventy-twelve",
    ];
    let successful_numbers = user_input
        .iter()
        .filter_map(|input| input.parse::<f32>().ok())
        .collect::<Vec<f32>>();
    println!("{:?}", successful_numbers);

    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Brendan McCracken"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];
    let results: Vec<Result<String, &str>> = company_vec
        .iter()
        .map(|company| company.get_ceo().ok_or("No CEO found"))
        .collect();
    for item in results {
        println!("{:?}", item);
    }

    let results: Vec<Result<String, String>> = company_vec
    .iter()
    .map(|company| {
        company.get_ceo().ok_or_else(|| {
            let err_message = format!("No CEO found for {}", company.name);
            println!("{err_message} at {}", get_current_datetime());
            err_message
        })
    })
    .collect();
    results
        .iter()
        .filter(|res| res.is_ok())
        .for_each(|res| println!("{res:?}"));

    let num_array = ["8", "9", "Hi", "9898989898"];
    let mut char_vec = vec![];
    for index in 0..5 {
        char_vec.push(
            num_array
            .get(index)
            .and_then(|number| number.parse::<u32>().ok())
            .and_then(|number| char::try_from(number).ok()),
        );
    }
    println!("{:?}", char_vec);

    for num in ["9", "nine", "ninety-nine", "9.9"]
        .into_iter()
        .map(|num| num.parse::<f32>())
    {
        println!("{num:?}");
    }

    for num in ["9", "nine", "ninety-nine", "9.9"]
        .into_iter()
        .map(|num| num.parse::<f32>())
        .flatten()
    {
        println!("{num}");
    }

    let char_vec = ('a'..'働').collect::<Vec<char>>();
    in_char_vec(&char_vec, 'i');
    in_char_vec(&char_vec, '뷁');
    in_char_vec(&char_vec, '鑿');
    let smaller_vec = ('A'..'z').collect::<Vec<char>>();
    println!(
        "All alphabetic? {}",
        smaller_vec.iter().all(|&x| x.is_alphabetic())
    );
    println!(
        "All less than the character 행? {}",
        smaller_vec.iter().all(|&x| x < '행')
    );

    let mut big_vec = vec![6; 1000];
    big_vec.push(5);
    let mut iterator = big_vec.iter().rev();
    assert_eq!(iterator.next(), Some(&5));
    assert_eq!(iterator.next(), Some(&6));

    let num_vec = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    println!("{:?}", num_vec.iter().find(|number| *number % 3 == 0));
    println!("{:?}", num_vec.iter().position(|number| *number % 3 == 0));
    println!("{:?}", num_vec.iter().find(|number| *number % 11 == 0));
    println!("{:?}", num_vec.iter().position(|number| *number % 11 == 0));

    let even_odd_iter = ["even", "odd"].into_iter().cycle();
    let even_odd_vec: Vec<(i32, &str)> = (0..=8)
        .zip(even_odd_iter)
        .collect();
    println!("{:?}", even_odd_vec);

    let ten_chars: Vec<char> = ('a'..).take(10).collect();
    let skip_then_ten_chars: Vec<char> = ('a'..).skip(1300).take(10).collect();
    println!("{ten_chars:?}");
    println!("{skip_then_ten_chars:?}");

    let some_numbers = vec![9, 6, 9, 10, 11];
    println!("{}", some_numbers
        .iter()
        .fold(0, |total_so_far, next_number| total_so_far + next_number)
    );

    let events = [
        "Went to grocery store",
        "Came home",
        "Fed cat",
        "Fed cat again",
    ];
    let empty_events = CombinedEvents {
        num_of_events: 0,
        data: vec![]
    };
    let combined_events =
        events
            .iter()
            .fold(empty_events, |mut total_events, next_event| {
                total_events.num_of_events += 1;
                total_events.data.push(next_event.to_string());
                total_events
            });
    println!("{combined_events:#?}");

    let mut number_iter = [7, 8, 9, 10].into_iter();
    let first_two = number_iter.by_ref().take(2).collect::<Vec<_>>();
    let second_two = number_iter.take(2).collect::<Vec<_>>();

    let num_vec = vec![1, 2, 3, 4, 5, 6, 7];
    for chunk in num_vec.chunks(3) {
        println!("{:?}", chunk);
    }
    println!();
    for window in num_vec.windows(3) {
        println!("{:?}", window);
    }

    let some_str = "Er ist noch nicht erklärt. Aber es gibt Krieg. Verlaß dich drauf.";
    for (index, item) in some_str.match_indices(|c| c > 'z') {
        println!("{item} at {index}");
    }
    for (index, item) in some_str.match_indices(". ") {
        println!("'{item}' at index {index}");
    }

    let mut my_number = dbg!(9);
    dbg!(my_number += 10);
    let new_vec = dbg!(vec![8, 9, 10]);
    let double_vec = dbg!(new_vec.iter().map(|x| x * 2).collect::<Vec<i32>>());
    dbg!(double_vec);

    let new_vec = vec![8, 9, 10];
    let double_vec = new_vec
    .iter()
    .inspect(|first_item| {
        println!("The item is: {first_item}");
        match **first_item % 2 {
        0 => println!("It is even."),
        _ => println!("It is odd."),
        }
        println!("In binary it is {:b}.", first_item);
    })
    .map(|x| x * 2)
    .collect::<Vec<i32>>();
}

#[derive(Debug)]
struct CombinedEvents {
    num_of_events: u32,
    data: Vec<String>,
}

fn in_char_vec(char_vec: &Vec<char>, check: char) {
    println!(
        "Is {check} inside? {}",
        char_vec.iter().any(|&char| char == check)
    );
}

struct Company {
    name: String,
    ceo: Option<String>,
}

fn get_current_datetime() -> String {
    "2024-01-27T23:11:23".to_string()
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string()),
        };

        Self {
            name: name.to_string(),
            ceo,
        }
    }
    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}
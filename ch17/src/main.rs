use serde::{Serialize, Deserialize};
use serde_json;

use std::time::{Instant, SystemTime, UNIX_EPOCH, Duration};
use std::thread::sleep;

fn main() {
    let good_json_request = r#"
    {
        "name": "BillyTheUser",
        "id": 6876
    }
    "#;
    let bad_json_request = r#"
    {
        "name": "BobbyTheUser",
        "idd": "6877"
    }
    "#;
    handle_request(good_json_request);
    handle_request(bad_json_request);

    let time = Instant::now();
    println!("{:?}", time);
    println!("{:?}", Instant::now());

    let start_of_main = Instant::now();
    let before_operation = Instant::now();

    let mut new_string = String::new();
    loop {
        new_string.push('წ');
        if new_string.len() > 100_000 {
            break;
        }
    }

    let after_operation = Instant::now();
    println!("{:?}", before_operation - start_of_main);
    println!("{:?}", after_operation - start_of_main);

    let start = Instant::now();
    println!("Time elapsed before busy operation: {:?}", start.elapsed());
    let mut new_string = String::new();
    loop {
        new_string.push('წ');
        if new_string.len() > 100_000 {
            break;
        }
    }
    println!("Operation complete. Time elapsed: {:?}", start.elapsed());

    bad_random_number(1);
    bad_random_number(1);
    bad_random_number(3);
    bad_random_number(3);

    bad_random_number(9);
    bad_random_number(9);
    bad_random_number(9);

    let instant = Instant::now();
    let system_time = SystemTime::now();
    println!("{instant:?}");
    println!("{system_time:?}");

    println!("{:?}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap());

    let three_seconds = Duration::from_secs(1);
    println!("I must sleep now.");
    sleep(three_seconds);
    println!("Did I miss anything?");
}

fn bad_random_number(digits: usize) {
    if digits > 9 {
        panic!("Random number can only be up to 9 digits");
    }
    let now_as_string = format!("{:?}", Instant::now());
    now_as_string
        .chars()
        .rev()
        .filter_map(|c| c.to_digit(10))
        .take(digits)
        .for_each(|character| print!("{}", character));
    println!();
}

#[derive(Debug, Serialize, Deserialize)]
struct NewUserRequest {
    name: String,
    id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    id: u32,
    is_deleted: bool,
}

impl From<NewUserRequest> for User {
    fn from(request: NewUserRequest) -> Self {
        Self {
            name: request.name,
            id: request.id,
            is_deleted: false,
        }
    }
}

fn handle_request(json_request: &str) {
    match serde_json::from_str::<NewUserRequest>(json_request) {
        Ok(good_request) => {
            let new_user = User::from(good_request);
            println!("Made a new user! {new_user:#?}");
            println!(
                "Serialized back into JSON: {:#?}",
                serde_json::to_string(&new_user)
            );
        }
        Err(e) => {
            println!("Got an error from {json_request}: {e}");
        }
    }
}
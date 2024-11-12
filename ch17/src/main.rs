use serde::{Serialize, Deserialize};
use serde_json;

//use std::io::{Error, ErrorKind};
use std::str::FromStr;
use std::time::{Instant, SystemTime, UNIX_EPOCH, Duration};
use std::thread::available_parallelism;
use std::thread::sleep;
use std::{num::ParseIntError, str::Utf8Error};

use chrono::naive::{NaiveDate, NaiveTime, NaiveDateTime};
use chrono::{DateTime, FixedOffset, Utc};

use rayon::prelude::*;

use anyhow::{anyhow, Context, Error};

use thiserror::Error;
use std::error::Error as StdError;

use std::collections::BTreeMap;

use std::sync::OnceLock;

//type MyError = thiserror::Error;

use reqwest::Client;
use std::sync::Mutex;

use lazy_static::lazy_static;

use once_cell::sync::OnceCell;


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

    println!("{:?}", NaiveDate::from_ymd_opt(2023, 3, 25));
    println!("{:?}", NaiveTime::from_hms_opt(12, 5, 30));
    println!("{:?}", NaiveDate::from_ymd_opt(2023, 3, 25).unwrap().and_hms_opt(12, 5, 30));

    for _ in 0..1000 {
        std::thread::spawn(|| {});
    }

    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let seconds = now.as_secs();
    println!("Seconds from 1970 to today: {seconds}");
    let naive_dt 
        = NaiveDateTime::from_timestamp_opt(seconds as i64, 0).unwrap();
    println!("As NaiveDateTime: {naive_dt}");
    let utc_dt = DateTime::<Utc>::from_utc(naive_dt, Utc);

    println!("As DateTime<Utc>: {utc_dt}");
    let kyiv_offset = FixedOffset::east_opt(3 * 60 * 60).unwrap();
    let kyiv_dt: DateTime::<FixedOffset> = DateTime::from_utc(naive_dt, kyiv_offset);
    println!("In a timezone 3 hours from UTC: {kyiv_dt}");
    let kyiv_naive_dt = kyiv_dt.naive_local();
    println!("With timezone information removed: {kyiv_naive_dt}");

    let incoming_event = UtcUserEvent {
        timestamp: "2023-03-27 23:48:50 UTC",
        data: "Something happened in UTC time".to_string(),
    };
    println!("Event as Utc:\n{incoming_event:?}");
    let korea_japan_event = KoreaJapanUserEvent::from(incoming_event);
    println!("Event in Korea/Japan time:\n{korea_japan_event:?}");

    let numbers: Vec<i32> = (1..10000).collect();

    // Sum the numbers in parallel
    let sum: i32 = numbers.par_iter().sum();
    println!("Sum: {}", sum);

    // Transform each element in parallel
    let squares: Vec<i32> = numbers.par_iter().map(|&x| x * x).collect();
    println!("Squares calculated in parallel for first 50: {:?}", &squares[..50]);

    let numbers: Vec<i32> = (1..10).collect();

    // Process each element in parallel
    numbers.par_iter().for_each(|&x| {
        println!("Processing number: {}", x);
    });

    let mut numbers = vec![5, 3, 8, 6, 2, 7, 4, 1];
    numbers.par_sort();
    println!("Sorted numbers: {:?}", numbers);

    let mut my_vec = vec![0; 2_000_000];
    my_vec
        .iter_mut()
        .enumerate()
        .for_each(|(index, number)| *number += index + 1);
    println!("{:?}", &my_vec[5000..5005]);

    let mut my_vec = vec![0; 2_000_000];
    my_vec
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, number)| *number += index + 1);
    println!("{:?}", &my_vec[5000..5005]);

    println!(
        "Estimated parallelism on this computer: {:?}",
        available_parallelism()
    );

    let mut without_rayon = vec![];
    let mut with_rayon = vec![];

    for _ in 0..10 {
        let mut my_vec = vec![0; 2_000_000];
        let now = std::time::Instant::now();
        my_vec.iter_mut().enumerate().for_each(|(index, number)| {
            *number += index + 1;
            *number -= index + 1;
        });
        let elapsed = now.elapsed();
        without_rayon.push(elapsed.as_micros());
        let mut my_vec = vec![0; 2_000_000];
        let now = std::time::Instant::now();
        my_vec
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, number)| {
                *number += index + 1;
                *number -= index + 1;
            });
        let elapsed = now.elapsed();
        with_rayon.push(elapsed.as_micros());
    }

    println!(
        "Average time without rayon: {} microseconds",
        without_rayon.into_iter().sum::<u128>() / 10
    );
    println!(
        "Average time with rayon: {} microseconds",
        with_rayon.into_iter().sum::<u128>() / 10
    );

    println!("{:?}", parse_then_send(b"nine"));
    println!("{:?}", parse_then_send(b"10"));

    println!("{}", parse_then_send2(b"nine").unwrap_err());
    println!("{}", parse_then_send2(&[8, 9, 0, 200]).unwrap_err());
    println!("{}", parse_then_send2(b"109080098").unwrap_err());
    println!("{}", SystemError::ColorError(8, 10, 200));
    parse_then_send2(b"10098").unwrap();

    8.hello();
    &'c'.hello();
    &mut String::from("Hello there").hello();
    8.7897.hello();
    Nothing.hello();
    std::collections::HashMap::<i32, i32>::new().hello();

    println!("{}", parse_then_send(b"nine").unwrap_err());
    println!("{:?}", parse_then_send(b"nine"));
    println!("{:?}", parse_then_send(b"10"));

    let my_btree_map = BTreeMap::from([
        ("customer_1_money".to_string(), 10),
        ("customer_2_money".to_string(), 200),
    ]);
    println!("{:?}", my_btree_map);

    GLOBAL_LOGGER.logs.lock().unwrap().push(Log {
        message: "Everything's going well".to_string(),
        timestamp: 1658930674
    });
    println!("{:#?}", *GLOBAL_LOGGER.logs.lock().unwrap());

    let url = fetch_url();
    GLOBAL_LOGGER2
        .set(Logger {
            logs: Mutex::new(vec![]),
            url,
            client: Client::default(),
        })
        .unwrap();
    GLOBAL_LOGGER2
        .get()
        .unwrap()
        .logs
        .lock()
        .unwrap()
        .push(Log {
            message: "Everything's going well".to_string(),
            timestamp: 1658930674,
        });
    println!("{GLOBAL_LOGGER2:?}");

    let url = fetch_url();
    GLOBAL_LOGGER3
        .set(Logger {
            logs: Mutex::new(vec![]),
            url,
            client: Client::default(),
        })
        .unwrap();
    GLOBAL_LOGGER3
        .get()
        .unwrap()
        .logs
        .lock()
        .unwrap()
        .push(Log {
            message: "Everything's going well".to_string(),
            timestamp: 1658930674,
        });
    println!("{GLOBAL_LOGGER3:?}");
}
//////////////////////////////

#[derive(Debug)]
struct Logger {
    logs: Mutex<Vec<Log>>,
    url: String,
    client: Client,
}
#[derive(Debug)]
struct Log {
    message: String,
    timestamp: i64,
}

// static GLOBAL_LOGGER: Logger = Logger {
//     logs: Mutex::new(vec![]),
//     url: "https://somethingsomething.com".to_string(),
//     client: Client::default()
// };

lazy_static! {
    static ref GLOBAL_LOGGER: Logger = Logger {
        logs: Mutex::new(vec![]),
        url: "https://somethingsomething.com".to_string(),
        client: Client::default()
    };
}

static GLOBAL_LOGGER2: OnceCell<Logger> = OnceCell::new();
static GLOBAL_LOGGER3: OnceLock<Logger> = OnceLock::new();

fn fetch_url() -> String {
    "http://somethingsomething.com".to_string()
}

#[derive(Error, Debug)]
enum SystemError2 {
    #[error("Couldn't send: {0}")]
    SendError(i32),
    #[error("External crate error: {0}")]
    ExternalCrateError(String),
}

trait ToSystemError<T> {
    fn to_system_error(self) -> Result<T, SystemError2>;
}

impl<T, E: StdError> ToSystemError<T> for Result<T, E> {
    fn to_system_error(self) -> Result<T, SystemError2> {
        self.map_err(|e| SystemError2::ExternalCrateError(e.to_string()))
    }
}

fn parse_then_send3(input: &[u8]) -> Result<(), SystemError2> {
    let some_str = std::str::from_utf8(input).to_system_error()?;
    let number = some_str.parse::<i32>().to_system_error()?;
    send_number3(number).to_system_error()?;
    Ok(())
}
    
fn send_number3(number: i32) -> Result<(), SystemError2> {
    if number < 1_000_000 {
        println!("Number sent!");
        Ok(())
    } else {
        println!("Too large!");
        Err(SystemError2::SendError(number))
    }
}

struct Nothing;

trait SaysHello {
    fn hello(&self) {
        println!("Hello");
    }
}

impl<T> SaysHello for T {}

#[derive(Error, Debug)]
enum SystemError {
    #[error("Couldn't send: {0}")]
    SendError(String),
    #[error("Couldn't parse into a str: {0}")]
    StringFromUtf8Error(#[from] Utf8Error),
    #[error("Couldn't turn into an i32: {0}")]
    ParseI32Error(#[from] ParseIntError),
    #[error("Wrong color: Red {0} Green {1} Blue {2}")]
    ColorError(u8, u8, u8),
    #[error("Something happened")]
    OtherError,
}

fn parse_then_send2(input: &[u8]) -> Result<(), SystemError> {
    let some_str = std::str::from_utf8(input)?;
    let number = some_str.parse::<i32>()?;
    send_number2(number)?;
    Ok(())
}

fn send_number2(number: i32) -> Result<(), SystemError> {
    match number {
        num if num == 500 => Err(SystemError::OtherError),
        num if num > 1_000_000 => Err(SystemError::SendError(format!(
            "{num} is too large, can't send!"
        ))),
        _ => {
            println!("Number sent!");
            Ok(())
        }
    }
}

fn parse_then_send(input: &[u8]) -> Result<(), Error> {
    // let some_str = std::str::from_utf8(input)?;
    // let number = some_str.parse::<i32>()?;
    // send_number(number)?;
    // Ok(())
    let some_str = std::str::from_utf8(input)
        .with_context(|| "Couldn't parse into a str")?;
    let number = some_str
        .parse::<i32>()
        .with_context(|| format!("Got a weird str to parse: {some_str}"))?;
    send_number(number)?;
    Ok(())
}

fn send_number(number: i32) -> Result<(), Error> {
    if number < 1_000_000 {
        println!("Number sent!");
        Ok(())
    } else {
        println!("Too large!");
        Err(anyhow!("Number is too large"))
    }
}

// fn parse_then_send0(input: &[u8]) {
//     let some_str = std::str::from_utf8(input)?;
//     let number = some_str.parse::<i32>()?;
//     //send_number(number)?;
// }

// fn send_number(number: i32) -> Result<(), Error> {
//     if number < 1_000_000 {
//         println!("Number sent!");
//         Ok(())
//     } else {
//         Err(String::from("Cannot divide by zero"))//Err(Error::new(ErrorKind::InvalidData))
//     }
// }

const SECONDS_IN_HOUR: i32 = 3600;
const UTC_TO_KST_HOURS: i32 = 9;
const UTC_TO_KST_SECONDS: i32 = UTC_TO_KST_HOURS * SECONDS_IN_HOUR;

#[derive(Debug)]
struct UtcUserEvent {
    timestamp: &'static str,
    data: String,
}

#[derive(Debug)]
struct KoreaJapanUserEvent {
    timestamp: DateTime<FixedOffset>,
    data: String,
}

impl From<UtcUserEvent> for KoreaJapanUserEvent {
    fn from(event: UtcUserEvent) -> Self {
        let utc_datetime: DateTime<Utc> = DateTime::from_str(event.timestamp).unwrap();
        let offset = FixedOffset::east_opt(UTC_TO_KST_SECONDS).unwrap();
        let timestamp: DateTime<FixedOffset> = DateTime::from_utc(utc_datetime.naive_utc(), offset);
        Self {
            timestamp,
            data: event.data,
        }
    }
}

#[test]
fn utc_to_korea_output_same_evening() {
    let morning_event = UtcUserEvent {
        timestamp: "2023-03-27 09:48:50 UTC",
        data: String::new(),
    };
    let to_korea_japan = KoreaJapanUserEvent::from(morning_event);
    assert_eq!(
        &to_korea_japan.timestamp.to_string(),
        "2023-03-27 18:48:50 +09:00"
    );
}

#[test]
fn utc_to_korea_output_next_morning() {
    let evening_event = UtcUserEvent {
        timestamp: "2023-03-27 23:59:59 UTC",
        data: String::new(),
    };
    let korea_japan_next_morning = KoreaJapanUserEvent::from(evening_event);
    assert_eq!(
        &korea_japan_next_morning.timestamp.to_string(),
        "2023-03-28 08:59:59 +09:00"
    );
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
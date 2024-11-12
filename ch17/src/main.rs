use serde::{Serialize, Deserialize};
use serde_json;

use std::str::FromStr;
use std::time::{Instant, SystemTime, UNIX_EPOCH, Duration};
use std::thread::sleep;

use chrono::naive::{NaiveDate, NaiveTime, NaiveDateTime};
use chrono::{DateTime, FixedOffset, Utc};

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
}
/////////

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
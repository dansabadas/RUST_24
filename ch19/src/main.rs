use reqwest::Client;
use tokio;

use std::time::Duration;
use rand::*;
use tokio::time::sleep;
use tokio::join;
use tokio::{select};
use tokio::try_join;

#[tokio::main]
async fn main() {
    let client = Client::default();
    //client.get("https://www.rust-lang.org").send().unwrap();
    //let response = client.get("https://www.rust-lang.org").send().await;

    // let client = reqwest::blocking::Client::default();
    // let response = client.get("https://www.rust-lang.org").send().unwrap();
    //println!("{}", response.unwrap());


    let some_number = async_give_8().await;

    let client = reqwest::Client::default();
    let response = client
        .get("https://www.rust-lang.org")
        .send()
        .await
        .unwrap();

    println!("{}", response.text().await.unwrap());

    let num1 = wait_and_give_u8(1).await;
    let num2 = wait_and_give_u8(2).await;
    let num3 = wait_and_give_u8(3).await;
    println!("{num1}, {num2}, {num3}");

    let nums = join!(
        wait_and_give_u8(1),
        wait_and_give_u8(2),
        wait_and_give_u8(3)
    );
    println!("{nums:?}");

    let num = select!(
        first = sleep_then_string(10) => first,
        second = sleep_then_string(11) => second,
        third = sleep_then_num(12) => format!("Slept for {third} millis!"),
        _ = sleep(Duration::from_millis(100)) => format!("Timed out after 100 millis!")
    );
    println!("{num}");

    let failed_join = try_join!(
        wait_then_u8(1, true),
        wait_then_u8(2, false),
        wait_then_u8(3, true)
    );
    let successful_join = try_join!(
        wait_then_u8(1, true),
        wait_then_u8(2, true),
        wait_then_u8(3, true)
    );
    println!("{failed_join:?}");
    println!("{successful_join:?}");
}

async fn wait_then_u8(num: u8, worked: bool) -> Result<u8, &'static str> {
    if worked {
        Ok(num)
    } else {
        Err("Oops, didn't work")
    }
}

async fn sleep_then_string(sleep_time: u64) -> String {
    sleep(Duration::from_millis(sleep_time)).await;
    format!("Slept for {sleep_time} millis!")
}

async fn sleep_then_num(sleep_time: u64) -> u64 {
    sleep(Duration::from_millis(sleep_time)).await;
    sleep_time
}

async fn wait_and_give_u8(num: u8) -> u8 {
    let mut rng = rand::thread_rng();
    let wait_time = rng.gen_range(1..100);
    sleep(Duration::from_millis(wait_time)).await;
    println!("Got a number! {num}");
    num
}

async fn async_give_8() -> u8 {
    8
}
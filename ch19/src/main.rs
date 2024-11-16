use reqwest::Client;

fn main() {
    let client = Client::default();
    //client.get("https://www.rust-lang.org").send().unwrap();

    let client = reqwest::blocking::Client::default();
    let response = client.get("https://www.rust-lang.org").send().unwrap();
    println!("{}", response.text().unwrap());
}

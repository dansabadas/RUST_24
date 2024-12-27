use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::path::Path;

fn main() {
    let outer = Arc::new(
        (Mutex::new(0), Condvar::new())
    );
    let inner = outer.clone();
    thread::spawn(move || {
        let (mutex, cond_var) = &*inner;
        let mut guard = mutex.lock().unwrap();
        *guard += 1;
        println!("inner guard={guard}");
        cond_var.notify_one();
    });
    let (mutex, cond_var) = &*outer;
    let mut guard = mutex.lock().unwrap();
    println!("outer before wait guard={guard}");
    while *guard == 0 {
        guard = cond_var.wait(guard).unwrap();
    }
    println!("outer after wait guard={guard}");

    let status = String::from("Active");
    let statuses = vec![status];
    println!("{:?}", statuses);
    //println!("{:?}", status);

    assert_eq!("abcdefg", reverse(String::from("gfedcba")));
    let str1 = "abcdefg".to_string();
    assert_eq!(
        reverse_and_uppercase(str1),
        ("gfedcba".to_string(), "GFEDCBA".to_string())
    );
    assert_eq!("abcdefg", reverse2("gfedcba"));

    let mut abcdefg = String::from("gfedcba");
    reverse_inplace(&mut abcdefg);
    assert_eq!("abcdefg", abcdefg);
    //println!("{:?}", str1);

    let pizza = Pizza::new0();
    println!("pizza={:?}", pizza);

    let pizza = Pizza::new(vec![
        String::from("tomato sauce"),
        String::from("mushrooms"),
        String::from("mozzarella"),
        String::from("pepperoni"),
    ]);
    println!("pizza={:#?}", pizza);

    let mut pub_pizza = Pizza {
        toppings: vec![String::from("sauce"), String::from("cheese")],
    };
    pub_pizza.toppings.remove(1);
    println!("pub_pizza={:?}", pub_pizza);

    let path = Path::new("Cargo.toml");
    let fourth_line = read_nth_line(Path::new("Cargo.toml"), 4);
    println!(
        "The 4th line from Cargo.toml reads: {:?}",
        fourth_line
    );
    let third_line = read_nth_line(Path::new("Cargo.toml"), 3);
    println!(
        "third_line: {:?}",
        third_line
    );
    let err = read_nth_line(Path::new("not-a-file"), 1);
    println!(
        "first_line: {:?}",
        err
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_read_cargotoml() {
        let third_line = read_nth_line(Path::new("Cargo.toml"), 3)
            .expect("unable to read third line from Cargo.toml");
        assert_eq!("version = \"0.1.0\"", third_line);
    }
    #[test]
    fn test_not_a_file() {
        let err = read_nth_line(Path::new("not-a-file"), 1)
            .expect_err("file should not exist");
        assert!(matches!(err, Error::Io(_)));
    }
    #[test]
    fn test_bad_arg_0() {
        let err = read_nth_line(Path::new("Cargo.toml"), 0)
            .expect_err("0th line is invalid");
        assert!(matches!(err, Error::BadLineArgument(0)));
    }
    #[test]
    fn test_bad_arg_too_large() {
        let err = read_nth_line(Path::new("Cargo.toml"), 500)
            .expect_err("500th line is invalid");
        assert!(matches!(err, Error::BadLineArgument(500)));
    }
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    BadLineArgument(usize),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

fn read_nth_line(path: &Path, n: usize) -> Result<String, Error> {
    if n < 1 {
        return Err(Error::BadLineArgument(0));
    }

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(path)?;
    let mut reader_lines = BufReader::new(file).lines();

    reader_lines
        .nth(n - 1)
        .map(|result| result.map_err(|err| err.into()))
        .unwrap_or_else(|| Err(Error::BadLineArgument(n)))
}

#[derive(Debug, Clone)]
pub struct Pizza {
    toppings: Vec<String>,
}

impl Pizza {
    pub fn new0() -> Self {
        Self { toppings: vec![] }
    }

    pub fn new(toppings: Vec<String>) -> Self {
        Self { toppings }
    }

    pub fn toppings(&self) -> &[String] {
        self.toppings.as_ref()
    }

    pub fn toppings_mut(&mut self) -> &mut Vec<String> {
        &mut self.toppings
    }

    pub fn set_toppings(&mut self, toppings: Vec<String>) {
        self.toppings = toppings;
    }

    pub fn replace_toppings(&mut self, toppings: Vec<String>) -> Vec<String> {
        std::mem::replace(&mut self.toppings, toppings)
    }
}

fn reverse_inplace(s: &mut String) {
    let mut v = Vec::from_iter(s.chars());
    v.reverse();
    s.clear();
    v.into_iter().for_each(|c| s.push(c));
}

fn reverse(s: String) -> String {
    let mut v = Vec::from_iter(s.chars());
    v.reverse();
    String::from_iter(v.iter())
}

fn reverse_and_uppercase(s: String) -> (String, String) {
    let mut v = Vec::from_iter(s.chars());
    v.reverse();
    let reversed = String::from_iter(v.iter());
    let uppercased = reversed.to_uppercase();
    (reversed, uppercased)
}

fn reverse2(s: &str) -> String {
    let mut v = Vec::from_iter(s.chars());
    v.reverse();
    String::from_iter(v.iter())
}
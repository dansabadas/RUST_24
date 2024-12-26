use std::sync::{Arc, Condvar, Mutex};
use std::thread;

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
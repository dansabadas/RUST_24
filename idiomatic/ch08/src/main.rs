use std::{borrow::Cow, cell::RefCell, collections::HashMap, marker::PhantomData, ops::Deref};

use uuid::Uuid;

use tiny_keccak::Hasher;
use tiny_keccak::keccakf;
use sha3::{Digest, Sha3_256};

fn main() {
    let session = Session::new();
    println!("{:?}", session);
    if let Ok(mut session) = session.authenticate("username", "password")
    {
        session.update_property("key", "value");
        println!("{:?}", session);
        let session = session.logout();
        println!("{:?}", session);
        
    }
    series_sum(3);
    let a = 1;
    let b = 2;
    mutability(a, b);
    dbg!(b);

    let immutable_string = String::from("This string cannot be changed");
    // immutable_string.push_str("... or can it?"); // error: cannot borrow `immutable_string` as mutable, as it is not declared as mutable
    dbg!(&immutable_string);
    let not_so_immutable_string = RefCell::from(immutable_string);
    not_so_immutable_string
        .borrow_mut()
        .push_str("... or can it?");
    dbg!(&not_so_immutable_string);
    //dbg!(&immutable_string);

    let cow_say_what = Cow::from("The cow goes moo");
    dbg!(&cow_say_what);
    let cows_dont_say_what =
    cow_say_what
        .clone()
        .to_mut()
        .replace("moo", "toot");
    dbg!(&cow_say_what);
    dbg!(&cows_dont_say_what);

    let ferris = Person("Ferris".to_string(), "Bueller".to_string(), 17);
    println!("Hello, {}!", *ferris);
    println!("The length of a person is {}", ferris.len());

    let dog = Dog::new("Rusty");
    let cat = Cat::new("Misty");

    println!("{} says: {}", dog.name(), dog.speak());
    println!("{} says: {}", cat.name(), cat.speak());

    // Input data
    let input = b"hello world";

    // Initialize the Keccak-256 hasher
    let mut hasher = Sha3_256::new();
    
    // Update the hasher with input data
    hasher.update(input);

    let result = hasher.finalize();
    let hash_hex = hex::encode(result);
    // Print the hash as a hex string
    println!("Keccak-256 hash: {:?}", hex::encode(hash_hex));

    let a: u32 = 7;
    let b = a&1;
    println!("{}", a>>1);
    println!("{}", countOneDigits(a));

    let name = "자우림";
    let other_name = String::from("Adrian Fahrenheit Țepeș");
    println!("My name is actually {}", other_name);
    println!("My name is actually {}", name);
    let name = " ";
    println!("My name is actually {}", name);

    let size_of_string = std::mem::size_of::<String>();
    let size_of_i8 = std::mem::size_of::<i8>();
    let size_of_f64 = std::mem::size_of::<f64>();
    let size_of_jaurim = std::mem::size_of_val("자우림");
    let size_of_adrian = std::mem::size_of_val("Adrian Fahrenheit Țepeș");
    println!("A String is Sized and always {size_of_string} bytes.");
    println!("An i8 is Sized and always {size_of_i8} bytes.");
    println!("An f64 is always Sized and {size_of_f64} bytes.");
    println!("But a &str is not Sized: '자우림' is {size_of_jaurim} bytes.");
    println!("And 'Adrian Fahrenheit Țepeș' is {size_of_adrian} bytes - not
    Sized.");

    let my_string = String::from("Try to make this a String111111111111111");
    let my_string: String = "Try to make this a String111111111111111".into();
    println!("my_string is {my_string} bytes.");
    let size_of_jaurim = std::mem::size_of_val(&my_string);
    println!("{my_string} is {size_of_jaurim} bytes.");

    let test = make_tester("hunter2");
    println!("{}", test("*******"));
    println!("{}", test("hunter2"));
}

async fn foo() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("done");
}

async fn bar() {
    let mut a = [0u8; 72];
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    for _ in 0..10 {
        a[0] += 1;
    }
    println!("done");
}

fn make_tester(answer: &str) -> impl Fn(&str) -> bool + '_ {
    move |challenge| {
        challenge == answer
    }
}

fn countOneDigits(a: u32) -> u32{
    let mut ret: u32 = 0;
    let mut b = a;
    loop {
        if b&1 == 1 {
            ret += 1;
        }
        b = b >> 1;
        if b == 0 {
            break;
        }
    }

    ret
}

fn range_bit_count(a: u32, b: u32) -> u32 {
    (a..=b).map(|i| i.count_ones()).sum()  // countOneDigits(i)
}

fn bmi(weight: u32, height: f32) -> &'static str {
    match weight as f32 / (height*height) {
        x if x <= 18.5 => "Underweight",
        x if x <= 25.0 => "Normal",
        x if x <= 30.0 => "Overweight",
        _ => "Obese"
    }
}

struct Animal {
    name: String,
}
    
impl Animal {
    fn new(name: &str) -> Animal {
        Animal { name: name.to_string() }
    }
    fn name(&self) -> &str {
        &self.name
    }
}

struct Dog(Animal);

impl Dog {
    fn new(name: &str) -> Self {
        Self(Animal::new(name))
    }
    fn speak(&self) -> &str {
        "Woof!"
    }
}
    
impl Deref for Dog {
    type Target = Animal;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
    
struct Cat(Animal);
    
impl Cat {
    fn new(name: &str) -> Self {
        Self(Animal::new(name))
    }
    fn speak(&self) -> &str {
        "Meow!"
    }
}

impl Deref for Cat {
    type Target = Animal;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Person(String, String, u32);

impl Deref for Person {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn is_square(n: i64) -> bool {
    (n as f64).sqrt().fract() == 0.0
    //let sqrt = (n as f64).sqrt() as i64;
    //n == sqrt * sqrt
}

fn is_square2(n: i64) -> bool {
    let mut i = 1;
    let mut num = n;
    loop {
        match num {
            num if num == 0 => {return true;},
            num if num < 0 => {return false;},
            _ => {
                num -= i;
                i += 2;
            }
        };
    }
}

fn string_to_array(s: &str) -> Vec<String> {
    s
        .split(' ')
        .map(|str| String::from(str))
        .collect()
}

fn double_char(s: &str) -> String {
    let mut ret = String::new();
    for i in s.chars(){
        ret.push(i);
        ret.push(i);
    }

    ret
}

fn get_grade(s1: u16, s2: u16, s3: u16) -> char {
    let avg = (s1 + s2 + s3) as f64/3.0;
    
    match avg {
        x if x <= 100.0 && x >= 90.0 => 'A',
        x if (80.0..90.0).contains(&x) => 'B',
        x if (70.0..80.0).contains(&x) => 'C',
        x if (60.0..70.0).contains(&x) => 'D',
        x if (0.0..60.0).contains(&x) => 'F',
        _ => 'F'
    }
}

fn mutability(
    a: i32, // immutable
    mut b: i32, // mutable
    ) {
    // a += 1; // error: cannot assign twice to immutable variable `a`
    b += 1;
    dbg!(a);
    dbg!(b);
}

fn series_sum(n: u32) -> String {
    // let mut sum: f64 = if n == 0 { 0.0 } else { 1.0 };
    // let mut cur_iter = 1;
    // for _ in 2..=n {
    //     cur_iter += 3;
    //     sum += 1.0/cur_iter as f64;
    // }

    // let stepped: Vec<u32> = (1..3*n+1).step_by(3).collect();
    // println!("{:?}", stepped);
    // let stepped2 = (1..3*n+1).step_by(3).fold(0.0, |acc, i| acc + 1.0/i as f64);
    // println!("{:?}", format!("{:.2}", stepped2));
    // format!("{:.2}", sum)
    format!("{:.2}", (1..3*n+1).step_by(3).fold(0.0, |acc, i| acc + 1.0/i as f64))
}

fn dna_to_rna(dna: &str) -> String {
    //dna.chars().map(|c| if c == 'T' { 'U' } else { c }).collect()
    dna.chars().map(|c| match c {
        'T' => 'U',
        _ => c
    }).collect()
}

pub trait SessionState {}

#[derive(Debug, Default)]
pub struct Session<State: SessionState = Initial> {
    session_id: Uuid,
    props: HashMap<String, String>,
    phantom: PhantomData<State>,
}

#[derive(Debug, Default)]
pub struct Initial;

#[derive(Debug, Default)]
pub struct Anonymous;

#[derive(Debug, Default)]
pub struct Authenticated;

#[derive(Debug, Default)]
pub struct LoggedOut;

impl SessionState for Initial {}
impl SessionState for Anonymous {}
impl SessionState for Authenticated {}
impl SessionState for LoggedOut {}

#[derive(Debug)]
pub enum ResumeResult {
    Invalid,
    Anonymous(Session<Anonymous>),
    Authenticated(Session<Authenticated>),
}

impl Session<Initial> {
    /// Returns a new session, defaulting to the anonymous state
    pub fn new() -> Session<Anonymous> {
        Session::<Anonymous> {
            session_id: Uuid::new_v4(),
            props: HashMap::new(),
            phantom: PhantomData,
        }
    }

    /// Returns the result of resuming this session from an existing ID.
    pub fn resume_from(session_id: Uuid)
        -> ResumeResult {
            ResumeResult::Authenticated(
                Session::<Authenticated> {
                    session_id,
                    props: HashMap::new(),
                    phantom: PhantomData,
                })
    }
}

impl Session<Anonymous> {
    pub fn authenticate(
        self,
        username: &str,
        password: &str,
    ) -> Result<Session<Authenticated>, Session<Anonymous>> {
        // ...
        if !username.is_empty() && !password.is_empty() {
            Ok(Session::<Authenticated> {
                session_id: self.session_id,
                props: HashMap::new(),
                phantom: PhantomData,
            })
        } else {
            Err(self)
        }
    }
}

impl Session<Authenticated> {
    pub fn update_property(&mut self, key: &str, value: &str) {
        if let Some(prop) = self.props.get_mut(key) {
            *prop = value.to_string();
        } else {
            self.props.insert(key.to_string(), value.to_string());
        }
        // ...
    }
    
    pub fn logout(self) -> Session<LoggedOut> {
            // ...
        Session {
            session_id: Uuid::nil(),
            props: HashMap::new(),
            phantom: PhantomData,
        }
    }
}        

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
      assert_eq!(dna_to_rna("TTTT"), "UUUU");
      assert_eq!(dna_to_rna("GCAT"), "GCAU");
    }

    fn test_strageSum(input: u32, expected: &str) {
        let actual = series_sum(input);
        assert!(actual == expected, "Expected series_sum({input}) to be {expected}, but was {actual}");
    }

    #[test]
    fn sample_tests() {
        test_strageSum(1, "1.00");
        test_strageSum(2, "1.25");
        test_strageSum(3, "1.39");
        test_strageSum(7, "1.68");
        test_strageSum(39, "2.26");
        test_strageSum(0, "0.00");
    }

    fn dotest_grade(s1: u16, s2: u16, s3: u16, expected: char) {
        let actual = get_grade(s1, s2, s3);
        assert!(actual == expected, 
            "With s1 = {s1}, s2 = {s2}, s = {s3}\nExpected '{expected}' but got '{actual}'")
    }
    
    #[test]
    fn test_get_grade() {
        dotest_grade(100,100,100, 'A');
        dotest_grade(95,90,93, 'A');
        dotest_grade(82,85,87, 'B');
        dotest_grade(60,82,76, 'C');
        dotest_grade(58,62,70, 'D');
        dotest_grade(58,59,60, 'F');
        dotest_grade(0,0,0, 'F');
    }

    fn do_stringtoarray_test(s: &str, expected: &[&str]) {
        let actual = string_to_array(s);
        assert!(actual == expected, "Test failed with s = \"{s}\"\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn stringtoarra_fixed_tests() {
        do_stringtoarray_test("Robin Singh", &["Robin", "Singh"]);
        do_stringtoarray_test("CodeWars", &["CodeWars"]);
        do_stringtoarray_test("I love arrays they are my favorite", &["I", "love", "arrays", "they", "are", "my", "favorite"]);
        do_stringtoarray_test("1 2 3", &["1", "2", "3"]);
    }

    #[test]
    fn basic_bmi_tests() {
        assert_eq!(bmi(50, 1.80), "Underweight");
        assert_eq!(bmi(80, 1.80), "Normal");
        assert_eq!(bmi(90, 1.80), "Overweight");
        assert_eq!(bmi(110, 1.80), "Obese");
    }

    fn dobittest(a: u32, b: u32, expected: u32) {
        let actual = range_bit_count(a, b);
        assert!(actual == expected, "With a = {a}, b = {b}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_bits_tests() {
        dobittest(2, 7, 11);
        dobittest(0, 1, 1);
        dobittest(4, 4, 1);
    }
}
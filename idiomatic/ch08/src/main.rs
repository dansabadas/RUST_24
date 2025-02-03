use std::{borrow::Cow, cell::RefCell, collections::HashMap, marker::PhantomData};

use uuid::Uuid;

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
}
//use bit_vec::BitVec;  

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use std::ptr; 

use num::integer::{lcm, gcd}; 

use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::error::Error;
//use aggregator::{Summary, Tweet};
//use {super::Summary, Tweet};
mod lib;

use lib::{Summary, Tweet, NewsArticle};

fn main() -> Result<(), Box<dyn Error>> {
    let holder = StringHolder {
        data: String::from("Struct-owned string"),
    };
    let s_ref = holder.get_reference();
    println!("Struct's string: {}", s_ref);

    let mut my_number = 8;
    let num_ref = &mut my_number;

    *num_ref += 10;
    println!("num_ref: {}", num_ref);

    let country = String::from("Austria");
    _ = print_country(country);
    //print_country(country);

    let country = String::from("Austria");
    print_country2(&country);
    print_country2(&country);

    let number = 9;
    let number_ref = &number;
    println!("{:p}", number_ref);
    println!("{:?}", br##"This will look like numbers"##);//I like to write "#".
    println!("{:?}", b"This will look like numbers");
    println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}");
    println!("{:X}", 'い' as u32);
    println!("ModularAddition(29, 87, 99)={}", ModularAddition(29, 87, 99));
    println!("u8_to_truncated_bits(9)={:?}", u8_to_truncated_bits(9));
    println!("ModularMultiplication(7, 8, 9)={}", ModularMultiplication(7, 8, 9));
    println!("ModularExponentiation(8, 2, 9)={}", ModularExponentiation(8, 2, 9));
    print!("{}\n", square_digits(9119));
    println!("ModularMultipliveInverse(2, 11)={}", ModularMultipliveInverse(2, 11));
    println!("ModularMultipliveInverse(3, 7)={}", ModularMultipliveInverse(3, 7));
    println!("ModularNegative(13, 29)={}", ModularNegative(13, 29));
    quarter_of(11);

    // Create and populate the HashMap  
    let mut my_map: HashMap<char, u8> = HashMap::new();  
    my_map.insert('a', 3);  
    my_map.insert('b', 2);  
    my_map.insert('c', 3);  
    my_map.insert('d', 1);  
  
    println!("HashMap: {:?}", my_map);  
  
    // Count how many values are equal to 3  
    let count = my_map.values().filter(|&&v| v == 3).count();  
  
    println!("Number of values equal to 3: {}", count);  
    println!("tribonacci(&[3., 2., 1.], 1): {:?}", tribonacci(&[1., 1., 1.], 1));  
    //&longest_consec(strarr, k), exp)
    println!("longest_consec: {}", longest_consec(vec!["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", "oocccffuucccjjjkkkjyyyeehh"], 1));  
    let kalmykia = Country {
        population: 500_000,
        capital: String::from("Elista"),
        leader_name: String::from("Batu Khasikov")
    };
    // let kalmykia2 = Country {
    //     500_000,
    //     String::from("Elista"),
    //     String::from("Batu Khasikov")
    // };
    //
    println!("next_item(&[Joe, Bob, Sally], Bob): {:?}", next_item(&["Joe", "Bob", "Sally"], "Bob")); 
    println!("delete_nth: {:?}", delete_nth(&[20,37,20,21], 1)); 

    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

    println!("add_binary: {:?}", add_binary(10, 4)); 
    println!("no_dangle: {:?}", no_dangle()); 

    let x = Box::new(2);
    let y = Box::new(&x);

    let i = ***y;
    println!("{:?} {:?} {:?}",x, y, i);

    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", v);

    let v1 = vec![1, 2, 3];
    let mut v2 = v1;
    v2.push(4);
    // println!("{}", v1[0]);
    //decipher_message("-@#--");
    decipher_message("^%iIHHh----#$!!---@#eEErAaDddd; -wWhoHHHOHo::-=rAreaA]\\-uoYoYoyuyoy");

    let s = String::from("Hello world");
    let s_ref = &s;
    //let s2 = *s_ref;
    //println!("{s2}");
    descending_order(12345);

    let string1 = String::from("hello");  
    let string2 = string1.clone(); // Same content, but a different memory location  
    let string3 = string1.as_str(); // Points to the same memory as `string1`  
  
    // Compare memory locations  
    if ptr::eq(string1.as_ptr(), string2.as_ptr()) {  
        println!("string1 and string2 point to the same memory location");  
    } else {  
        println!("string1 and string2 point to different memory locations");  
    }  
  
    // Compare memory locations for string1 and string3  
    if ptr::eq(string1.as_ptr(), string3.as_ptr()) {  
        println!("string1 and string3 point to the same memory location");  
    } else {  
        println!("string1 and string3 point to different memory locations");  
    } 

    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>(),
      );

    let v = vec![1, 2, 3];
    let v_ref: &Vec<i32> = &v;
    // let v2 = *v_ref;

    println!("{}", lcm(lcm(2,2), 4));
    sum_fracts(vec![(10, 20), (100, 300), (10, 40)]);

    let v = vec![1, 2, 3, 4, 5];

    //let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    //v.push(6);

    println!("The first element is: {first}");
    v.push(6);
    println!("The whole is: {v:?}");

    let mut v = vec![100, 32, 57];
    for n_ref in &mut v {
        // n_ref has type &mut i32
        *n_ref += 50;
    }
    println!("The whole is: {v:?}");

    let mut v: Vec<i32> = vec![1, 2, 3];
    let mut v2: Vec<&mut i32> = Vec::new();
    for i in &mut v {
        v2.push(i);
    }
    *v2[0] = 5;
    let a = *v2[0];
    let b = v[0];
    println!("{a} {b}");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {s}");

    let hello = "Здравствуйте";

    let s = &hello[0..2];//1 fails, 4 displays 2 chars
    println!("s is {s}");
    sum_intervals(&[(1, 5), (6, 10)]);

    let v = vec![1, 2, 3];

    //v[99];

    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        //Err(error) => panic!("Problem opening the file: {error:?}"),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
    let greeting_file = File::open("hello.txt").unwrap();
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    let text = read_username_from_file2();
    match text {
        Ok(str) => print!("{}", str),
        Err(err) => print!("{:?}", err),
    }
   //print!("{}", text.unwrap());
   let f1 = last_char_of_first_line(" \nhi");
   print!("{:?}", f1);

   let greeting_file = File::open("hello.txt")?;

   println!("{:?}", remove_char("12345"));

   let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize2());

    // let r;

    // {
    //     let x = 5;
    //     //r = &x;
    // }

    // println!("r: {}", r);
    println!("arrange {}", arrange("who hit retaining The That a we taken"));

    for (i, j) in (0..5).zip(1..5) {
        println!("arrange {:?}", (i,j));
    }

    let string1 = String::from("long string is long");
    let str1 = "long string is long";
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        let result = longest(str1, "xyz");
        println!("The longest string is {result}");
    }

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("i is {i:?}");

    let s: &'static str = "I have a static lifetime.";
    Ok(())
}

fn positive_sum(slice: &[i32]) -> i32 {
    // your code
    slice.iter().filter(|x| **x > 0).sum()
}

struct Foo<'a> {
    bar: &'a i32
}

fn baz<'a, 'b>(f: &'a Foo<'b>) -> &'b i32 { &f.bar }

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]  
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn shortest<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() < y.len() {
      x
    } else {
      y
    }
  }

fn longest<'c>(x: &'c str, y: &'c str) -> &'c str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn arrange(s: &str) -> String {
    let mut ret = s.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>();
    for i in 0..ret.len()-1 {
        if ret[i] > ret[i+1] && i%2==1 {
            let tmp = ret[i].clone();
            ret[i] = ret[i+1].clone();
            ret[i+1] = tmp;
        } else if ret[i] < ret[i+1] && i%2==0 {
            let tmp = ret[i].clone();
            ret[i] = ret[i+1].clone();
            ret[i+1] = tmp;
        }
    }
    ret.join(" ")
}

fn solution(phrase: &str) -> String {
    phrase.chars().into_iter().rev().collect()
}

fn returns_summarizable2(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        NewsArticle {
            headline: String::from(
                "Penguins2 win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
        // Tweet {
        //     username: String::from("horse_ebooks"),
        //     content: String::from(
        //         "of course, as you probably already know, people",
        //     ),
        //     reply: false,
        //     retweet: false,
        // }
    }
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn remove_char(s: &str) -> String {
    s[1..s.len()-1].to_string()
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    let lml = username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn sum_intervals(intervals: &[(i32, i32)]) -> i32 {
    let mut ret: Vec<(i32,i32)> = Vec::new();
    let calculate_intersection = |i1: &(i32, i32), i2: &(i32, i32)| {
        if i1.0 >= i2.0 && i1.0 <= i2.1 {
            if i1.1 <= i2.1 {
                return i1.1 - i1.0;
            } else {
                return i2.1 - i1.0;
            }
        } else if i2.0 >= i1.0 && i2.0 <= i1.1 {
            if i1.1 <= i2.1 {
                return i1.1 - i2.0;
            } else {
                return i2.1 - i2.0;
            }
        }

        -1
    };
    let mut prev_interval = (0,0);
    for (start,end) in intervals {
        for (s, e) in ret.iter() {

        }
        ret.push((*start, *end));
    }
    println!("s is {ret:?}");
    2
}

fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    data.into_iter().map(|t| {
        match t{
            (age, handicap) if age >= 55 && handicap > 7 => String::from("Senior"),
            _ => String::from("Open")
        }
    }).collect()
  }

fn sum_fracts(l: Vec<(i64, i64)>) -> Option<(i64, i64)> {
    let mut lcm_var = 1i64;
    let mut numerators_vec: Vec<i64> = l.iter().map(|t| t.0).collect();
    // if let Some(_) = numerators_vec.iter().find(|x| **x == 0) {
    //     return None;
    // } 
    for t in l.iter() {
        lcm_var = lcm(lcm_var, t.1)
    }
    for (i, val) in numerators_vec.iter_mut().enumerate() {
        *val *= lcm_var/(l[i].1);
    }

    let sum_numerators = numerators_vec.iter().sum::<i64>();
    let gcd_var = gcd(sum_numerators, lcm_var);

    Some((sum_numerators/gcd_var, lcm_var/gcd_var))
}

fn bouncing_ball(h: f64,  bounce: f64,  window: f64) -> i32 {
    if h <= 0. { return -1; }
    if bounce <= 0. || bounce >= 1. { return -1; }
    if window >= h { return -1; }
    
    let mut no_times: i32 = 1;
    let mut h = h;
    loop {
        h *= bounce;
        if h <= window { break; }
        no_times += 2;
    }

    no_times
}

fn descending_order(x: u64) -> u64 {
    x
        .to_string()
        .chars()
        .sorted()
        .rev()
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

// fn copy_to_prev(v: &mut Vec<i32>, i: usize) {
//     let n = &mut v[i];
//     *n = v[i - 1];
// }

pub fn decipher_message(code: &str) -> String {
    let analyze_word = |input_word: String| {
        let mut ret: Vec<(char, u8)> = Vec::new();
        for c in input_word.chars() {
            if !c.is_ascii_alphabetic() {
                continue;
            }

            if let Some(t) = ret.iter_mut().find(|tuple| tuple.0 == c) {
                t.1 += 1;
            } else {
                ret.push((c, 1));
            }
        }
        ret.sort_by(|a, b| b.1.cmp(&a.1));
        ret.iter().map(|t| t.0).join("")
    };

    let vecret = code.to_ascii_uppercase().split('-')
        .map(|word| analyze_word(word.to_string()))
        .filter(|word| !word.is_empty())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{:?}", vecret);

    vecret.to_string()
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn beggars(values: &[u32], n: usize) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }
    let mut ret = vec![0u32; n];
    for (i, &val) in values.iter().enumerate() {
        ret[i % n] += val;
    }

    ret
}

fn add_binary(a: u64, b: u64) -> String {
    format!("{:b}",(a+b))
}

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();
    let mut repeat_hash: HashMap<u8, u8> = HashMap::new();
    let n: u8 = n as u8;
    for &i in lst {
        let count = repeat_hash.entry(i).or_insert(0);
        if *count < n {
            ret.push(i);
            *count += 1;
        }
        // if let  Some(val) = repeat_hash.get(i) {
        //     if *val < n {
        //         ret.push(*i);
        //     }
        // } else {
        //     ret.push(*i);
        // }
        // *repeat_hash.entry(*i).or_insert(0) += 1;
    }

   ret
}

fn count_by(x: u32, n: u32) -> Vec<u32> {
    (1..=n).map(|i| i*x).collect()
}

fn collinearity(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    if x1 == 0 && y1 == 0 || x2 == 0 && y2 == 0 { return true; }
    if x1 == 0 && x2 == 0 { return true; }
    if y1 == 0 && y2 == 0 { return true; }
    if (y1 as f32/x1 as f32) == (y2 as f32/x2 as f32) { return true; }
    false
}

fn next_item<T: PartialEq<T> + Clone>(slice: &[T], find: T) -> Option<T> {
    match slice.iter().position(|x| *x == find) {
        Some(index) => { 
            match index != slice.len()-1 {
                true => Some(slice[index+1].clone()),
                false => None
            }
        },
        None => None
    }
}

struct Country {
    population: u32,
    capital: String,
    leader_name: String
}

fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    if k > strarr.len() || k == 0 || strarr.len() == 0 { String::new() } else {
        strarr.windows(k).map(|x| { x.join("") }).max_by_key(String::len).unwrap()
    }
}

fn longest_consec2(strarr: Vec<&str>, k: usize) -> String {
    if strarr.len() == 0 || k > strarr.len() || k <= 0 { return String::from(""); }
    let mut biggest_string = String::new();
    let mut biggest_length = 0; 
    for i in 0..strarr.len() - k + 1 {
        let curr_str = strarr[i..k+i].join("");
        if curr_str.len() > biggest_length {
            biggest_length = curr_str.len();
            biggest_string = curr_str;
        }
    }
    
    biggest_string.clone()
}

fn tribonacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
    let mut ret: Vec<f64> = vec![0.; n];

    for i in 0..signature.len() {
        if i < n {
            ret[i] = signature[i];
        }
    }

    for i in 3..n {
        ret[i] = ret[i-3] + ret[i-2] + ret[i-1];
    }

    ret
}

fn get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;
  
    for c in string.chars() {
        if "aeiou".contains(c){
            vowels_count += 1;
        }
        // match c {
        //     'a' => { vowels_count += 1; },
        //     'e' => { vowels_count += 1; },
        //     'i' => { vowels_count += 1; },
        //     'o' => { vowels_count += 1; },
        //     'u' => { vowels_count += 1; },
        //     _ => {}
        // }
    }
    
    vowels_count
}

fn count_duplicates(text: &str) -> u32 {
    let mut repeated_chars: Vec<char> = Vec::new(); 
    let mut ret = HashSet::new(); 
    for c in text.chars() {
        if repeated_chars.contains(&c){
            ret.insert(c);
        } else {
            repeated_chars.push(c);
        }
    }

    ret.len() as u32
}

fn quarter_of(month: u8) -> u8 {
    (month-1)/3 + 1
}

fn get_age(age: &str) -> u32 {
    age.chars().next().unwrap().to_string().parse::<u32>().unwrap()
}

fn ModularAddition(a: u8, b: u8, n: u8) -> u8{
    //return c = a + b (mod n)
    match n - a {
        x if x > b => a + b,
        x if x <= b => b - x,
        _ => panic!()
    }
}

fn ModularNegative(a: u8, n: u8) -> u8{
    //return c = -a (mod n)
    
    n-a
}

fn u8_to_truncated_bits(input: u8) -> Vec<u8> {  
    let mut bits = Vec::new();  
    let mut found_one = false; // Flag to skip leading zeroes  
  
    for i in (0..8).rev() { // Iterate from most significant bit to least significant bit  
        let bit = (input >> i) & 1; // Extract the bit  
        if bit == 1 {  
            found_one = true; // Start collecting once we see the first `1`  
        }  
        if found_one {  
            bits.push(bit);  
        }  
    }  
  
    if bits.is_empty() {  
        bits.push(0); // Special case: input is 0, return a single 0  
    }  
  
    bits  
} 

fn ModularMultiplication(a: u8, b: u8, n: u8) -> u8 {
    //return c = a * b (mod n)
    let A = u8_to_truncated_bits(a);
    let mut r = 0;
    let mut s = b;

    for i in A.iter().rev() {
        if i == &1 {
            r = ModularAddition(r, s, n);
        }
        s = ModularAddition(s, s, n);
    }
    
    r
}

fn ModularExponentiation(a: u8, e: u8, n: u8) -> u8 {
     //return c = a ^ e (mod n)
    let A = u8_to_truncated_bits(e);
    let mut r = 1;
    let mut m = a;

    for i in A.iter().rev() {
        if i == &1 {
            r = ModularMultiplication(r, m, n);
        }
        m = ModularMultiplication(m, m, n);
    }
    
    r
}

fn ModularMultipliveInverse(a: u8, n: u8) -> u8 {
    //return c = 1/a (mod n) where gcd(a, n) = 1  
    //Ferman't little theorem <=> a^(n-1)~1(mod n)

    ModularExponentiation(a, n-2, n)
}

fn reverse_words(str: &str) -> String {
    // your code here
    str
        .split(' ')   
        .map(|w| w.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

fn first_non_consecutive(arr: &Vec<i32>) -> Option<i32> {
    // let mut currentlyIterated = arr[0] - 1;
    // for &i in arr {
    //     if i != (currentlyIterated + 1) { return Some(i); }
    //     currentlyIterated = i;
    // }

    for i in 1..=arr.len()-1 {
        if arr[i] != (arr[i-1] + 1) { return Some(arr[i]) }
    }

    None
}

fn print_country2(country_name: &String) {
    println!("{}", country_name);
}

fn print_country(country_name: String) -> String {
    println!("{}", country_name);
    country_name
}

struct StringHolder {
    data: String,
}

impl StringHolder {
    // Returns a reference to the String owned by the struct
    fn get_reference(&self) -> &String {
        &self.data
    }
}

//static MOO: String = "Austria".into();

// fn return_str() -> &String {
//     let country = String::from("Austria");
//     let country_ref = &country;
//     country_ref
// }//

fn square_digits(num: u64) -> u64 {
    let mut num = num;
    let mut digits = Vec::new();  
    loop {
        let digit = num % 10;
        digits.push((digit*digit).to_string());
        num = num / 10;
        if num == 0 {break;}
    }
    digits.reverse();

    digits.join("").parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_square_digits() {
        assert_eq!(square_digits(9119), 811181, "\nFailed with num 9119");
    }

    #[test]
    fn test_consecutivevec_basic() {
        assert_eq!(first_non_consecutive(&vec![1,2,3,4,6,7,8]), Some(6));
        assert_eq!(first_non_consecutive(&vec![1,2,3,4,5,6,7,8]), None);
        assert_eq!(first_non_consecutive(&vec![4,6,7,8,9,11]), Some(6));
        assert_eq!(first_non_consecutive(&vec![4,5,6,7,8,9,11]), Some(11));
        assert_eq!(first_non_consecutive(&vec![31,32]), None);
        assert_eq!(first_non_consecutive(&vec![-3,-2,0,1]), Some(0));
        assert_eq!(first_non_consecutive(&vec![-5,-4,-3,-1]), Some(-1));
    }

    #[test]
    fn sample_test2() {
        assert_eq!(reverse_words("The quick brown fox jumps over the lazy dog."), "ehT kciuq nworb xof spmuj revo eht yzal .god");
        assert_eq!(reverse_words("apple"), "elppa");
        assert_eq!(reverse_words("a b c d"),"a b c d");
        assert_eq!(reverse_words("double  spaced  words"), "elbuod  decaps  sdrow");
    }

    #[test]
    fn basic_age_tests() {
        assert_eq!(get_age("2 years old"), 2);
        assert_eq!(get_age("4 years old"), 4);
        assert_eq!(get_age("5 years old"), 5);
        assert_eq!(get_age("7 years old"), 7);
    }

    #[test]
    fn basic() {
        assert_eq!(quarter_of(3), 1, "Month 3 = quarter 1");
        assert_eq!(quarter_of(8), 3, "Month 8 = quarter 3");
        assert_eq!(quarter_of(11), 4, "Month 11 = quarter 4");
    }  

    #[test]
    fn test_duplicates() {
        assert_eq!(count_duplicates("abcde"), 0);
        assert_eq!(count_duplicates("abcdea"), 1);
        assert_eq!(count_duplicates("indivisibility"), 1);
    }

    #[test]
    fn my_tests_vowels_count() {
        assert_eq!(get_count("abracadabra"), 5);
    }

    #[test]
    fn basic_tests_tribonacci1() {
        assert_eq!(tribonacci(&[0., 1., 1.], 10), vec![0., 1., 1., 2., 4., 7., 13., 24., 44., 81.]);
        assert_eq!(tribonacci(&[1., 0., 0.], 10), vec![1., 0., 0., 1., 1., 2., 4., 7., 13., 24.]);
        assert_eq!(tribonacci(&[0., 0., 0.], 10), vec![0., 0., 0., 0., 0., 0., 0., 0., 0., 0.]);
        assert_eq!(tribonacci(&[1., 2., 3.], 10), vec![1., 2., 3., 6., 11., 20., 37., 68., 125., 230.]);
        assert_eq!(tribonacci(&[3., 2., 1.], 10), vec![3., 2., 1., 6., 9., 16., 31., 56., 103., 190.]);
        assert_eq!(tribonacci(&[1., 1., 1.], 1), vec![1.]);
        assert_eq!(tribonacci(&[300., 200., 100.], 0), vec![]);
        assert_eq!(tribonacci(&[0.5, 0.5, 0.5], 30), vec![0.5, 0.5, 0.5, 1.5, 2.5, 4.5, 8.5, 15.5, 28.5, 52.5, 96.5, 177.5, 326.5, 600.5, 1104.5, 2031.5, 3736.5, 6872.5, 12640.5, 23249.5, 42762.5, 78652.5, 144664.5, 266079.5, 489396.5, 900140.5, 1655616.5, 3045153.5, 5600910.5, 10301680.5]);
    }

    fn testing(strarr: Vec<&str>, k: usize, exp: &str) -> () {
        assert_eq!(&longest_consec(strarr, k), exp)
    }

    #[test]
    fn basics_longest_consec() {
        testing(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2, "abigailtheta");
        testing(vec!["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", "oocccffuucccjjjkkkjyyyeehh"], 1, 
            "oocccffuucccjjjkkkjyyyeehh");
        testing(vec![], 3, "");
        testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 3, "ixoyx3452zzzzzzzzzzzz");
        testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
        testing(vec!["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
    }

    #[test]
    fn returns_expected() {
    assert_eq!(next_item(&["Joe", "Bob", "Sally"], "Bob"), Some("Sally"));
    assert_eq!(next_item(&[0, 1], 0), Some(1));
    assert_eq!(next_item(&[0, 1], 1), None);
    assert_eq!(next_item((1..10).collect::<Vec<_>>().as_slice(), 7), Some(8));  
    }

    fn do_colin_test(x1: i32, y1: i32, x2: i32, y2: i32, expected: bool) {
        assert_eq!(
            collinearity(x1, y1, x2, y2),
            expected,
            "Input: ({x1}, {y1}, {x2}, {y2})"
        );
    }

    #[test]
    fn test_fixed_one_direction() {
        do_colin_test(1, 1, 1, 1, true);
        do_colin_test(1, 2, 2, 4, true);
        do_colin_test(1, 2, 3, 7, false);
    }

    #[test]
    fn test_fixed_opposite_direction() {
        do_colin_test(1, 1, 6, 1, false);
        do_colin_test(1, 2, -1, -2, true);
        do_colin_test(1, 2, 1, -2, false);
    }

    #[test]
    fn test_fixed_vectors_contain_zero() {
        do_colin_test(4, 0, 11, 0, true);
        do_colin_test(0, 1, 6, 0, false);
        do_colin_test(4, 4, 0, 4, false);
    }

    #[test]
    fn test_fixed_vector_with_0_0_coordinates() {
        do_colin_test(0, 0, 0, 0, true);
        do_colin_test(0, 0, 1, 0, true);
        do_colin_test(5, 7, 0, 0, true);
    }


    
    #[test]
    fn sample_tests() {
        assertion(vec![1,2,3,4,5,6,7,8,9,10], (1, 10));
        assertion(vec![2,4,6,8,10], (2, 5));
        assertion(vec![3,6,9,12,15,18,21], (3, 7));
        assertion(vec![50,100,150,200,250], (50, 5));
        assertion(vec![100,200,300,400,500,600], (100, 6));
    }
    
    fn assertion(expected : Vec<u32>, inputs : (u32, u32)) {
        let actual = count_by(inputs.0, inputs.1);
        
        assert!(
            expected == actual,
            "\nTest failed!\n expected: [{}]\n actual: [{}]\n x: {}\n n: {}\n"
            , expected.iter().join(", ")
            , actual.iter().join(", ")
            , inputs.0
            , inputs.1
        );
    }

    #[test]
    fn test_basic() {
        assert_eq!(delete_nth(&[20,37,20,21], 1), vec![20,37,21]);
        assert_eq!(delete_nth(&[1,1,3,3,7,2,2,2,2], 3), vec![1, 1, 3, 3, 7, 2, 2, 2]);
    }

    #[test]
    fn test_basic2() {
        assert_eq!(beggars(&[1, 2, 3, 4, 5], 1), [15]);
        assert_eq!(beggars(&[1, 2, 3, 4, 5], 2), [9, 6]);
        assert_eq!(beggars(&[1, 2, 3, 4, 5], 3), [5, 7, 3]);
        assert_eq!(beggars(&[1, 2, 3, 4, 5], 6), [1, 2, 3, 4, 5, 0]);
    }
    
    #[test]
    fn test_zero_beggars() {
        assert_eq!(beggars(&[1, 2, 3, 4, 5], 0), []);
    }

    #[test]
    fn empty() {
        assert_eq!(decipher_message(""), "".to_string());
    }
    #[test]
    fn just_noise() {
        assert_eq!(decipher_message("#%^!@"), "".to_string());
        assert_eq!(decipher_message("     "), "".to_string());
    }
    #[test]
    fn just_separators() {
        assert_eq!(decipher_message("-"), "".to_string());
        assert_eq!(decipher_message("----"), "".to_string());
    }
    #[test]
    fn just_noise_with_separators() {
        assert_eq!(decipher_message("@#-"), "".to_string());
        
        assert_eq!(decipher_message("-@#--"), "".to_string());
        assert_eq!(decipher_message("-@#--"), "".to_string());
    }
    #[test]
    fn plain_all_capitalized() {
        assert_eq!(decipher_message("MMMMAAAKKE"), "MAKE".to_string());
        assert_eq!(decipher_message("SPPAAA"), "APS".to_string());
        assert_eq!(decipher_message("OONNNT"), "NOT".to_string());
        assert_eq!(decipher_message("RWWWWWWWWAAAA"), "WAR".to_string());
        assert_eq!(decipher_message("REEEAADDDD"), "DEAR".to_string());
    }
    #[test]
    fn plain_different_case() {
        assert_eq!(decipher_message("HHhIi"), "HI".to_string());
        assert_eq!(decipher_message("IihHh"), "HI".to_string());
        assert_eq!(decipher_message("DDDDddddEEEEeeeAAaR"), "DEAR".to_string());
        assert_eq!(decipher_message("eEErAaDddddd"), "DEAR".to_string());
    }
    #[test]
    fn different_case_words_with_noise() {
        assert_eq!(decipher_message("iIHHh#$"), "HI".to_string());
        assert_eq!(decipher_message(" ^%iIHHh"), "HI".to_string());
        assert_eq!(decipher_message("^%iIHHh#$ "), "HI".to_string());
        assert_eq!(decipher_message("!!@#eEErAaDddd;"), "DEAR".to_string());
        assert_eq!(decipher_message("!!@#eEEr@@!& AaDddd;"), "DEAR".to_string());
    }
    #[test]
    fn different_case_words_with_noise_in_a_short_sentence() {
        assert_eq!(decipher_message("^%iIH Hh#$-!!@#eEErAaDddd;"), "HI DEAR".to_string());
        assert_eq!(decipher_message("^%iIHHh--- --#$!!@#eEErAaDddd;"), "HI DEAR".to_string());
        assert_eq!(decipher_message("^%iIHHh----#$!!---@#eEErAaDddd; -wWhoHHHOHo::-=rAreaA]\\-uoYoYoyuyoy"), "HI DEAR HOW ARE YOU".to_string());
    }
    #[test]
    fn shuffled_different_case_words_with_noise_in_a_short_sentence() {
        assert_eq!(decipher_message("^I%Hh #iH$-!eE!D@Edrd A#ad;"), "HI DEAR".to_string());
        assert_eq!(decipher_message("^% iIHHh- ---#$!!---@#eEErAaD ddd;"), "HI DEAR".to_string());
        assert_eq!(decipher_message("^I#H ?iH+h----#$!!---@#eE ErAaDddd;"), "HI DEAR".to_string());
        assert_eq!(decipher_message("^%iIHHh----#$!!---@#eEErAaDddd; -wWhoHH?/\\ ::H?=OHo::-=rAr@eaA]\\-u,<oYoYoyuyoy"), "HI DEAR HOW ARE YOU".to_string());
    }

    #[test]
    fn returns_expected_3() {
        assert_eq!(descending_order(0), 0);
        assert_eq!(descending_order(1), 1);
        assert_eq!(descending_order(15), 51);
        assert_eq!(descending_order(1021), 2110);
        assert_eq!(descending_order(123456789), 987654321);
        assert_eq!(descending_order(145263), 654321);
        assert_eq!(descending_order(1254859723), 9875543221);
    }

    fn testequal2(h: f64,  bounce: f64,  window: f64, exp: i32) -> () {
        assert_eq!(bouncing_ball(h, bounce, window), exp)
    }
    
    #[test]
    fn tests_bouncing_ball() {
    
        testequal2(3.0, 0.66, 1.5, 3);
        testequal2(30.0, 0.66, 1.5, 15);
        testequal2(40.0, 0.4, 10.0, 3);
        testequal2(10.0, 0.6, 10.0, -1);
      
    }

    fn testing2(l: Vec<(i64, i64)>, exp: Option<(i64, i64)>) -> () {
        assert_eq!(sum_fracts(l), exp)
    }
    
    #[test]
    fn tests_sum_fracts() {
        testing2(vec![(1, 2), (1, 3), (1, 4)], Some((13, 12))); 
        testing2(vec![(1, 3), (5, 3)], Some((2, 1)));
        testing2(vec![(10, 20), (100, 300), (10, 40)], Some((13, 12))); 
        //testing2(vec![(0, 20)], None); 
        //testing2(vec![(0, 1), (10, 20), (100, 300), (10, 40)], Some((13, 12))); 
    }

    fn returns_expected3() {
        assert_eq!(open_or_senior(vec![(45, 12), (55,21), (19, -2), (104, 20)]), vec!["Open", "Senior", "Open", "Senior"]);
        assert_eq!(open_or_senior(vec![(3, 12), (55,1), (91, -2), (54, 23)]), vec!["Open", "Open", "Open", "Open"]);
    }

    // fn testing_str(s: &str, exp: &str) -> () {
    //     assert_eq!(arrange(s), exp.to_string())
    // }
    
    // #[test]
    // fn basics_arrange() {
    
    //     testing_str("who hit retaining The That a we taken", "who RETAINING hit THAT a THE we TAKEN"); // 3
    //     testing_str("on I came up were so grandmothers", "i CAME on WERE up GRANDMOTHERS so"); // 4
    // }
}
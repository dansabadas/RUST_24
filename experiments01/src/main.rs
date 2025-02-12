//use bit_vec::BitVec;  

use std::collections::{HashMap, HashSet};

fn main() {
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
}
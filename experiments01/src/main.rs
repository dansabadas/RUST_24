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
}
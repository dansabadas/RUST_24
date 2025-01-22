use std::{ops::Deref, vec::IntoIter};

fn main() {
    println!("Hello, world!");
    let buf = Buffer::from([0, 1, 2, 3]);
    dbg!(&buf);
    println!("{}",sum_two_smallest_numbers(&[15, 28, 4, 2, 43]));
    println!("{}",get_middle("testing"));
    println!("{}",&"testing"[1..2]);

    let wrapped_vec = WrappedVec(vec![1, 2, 3]);
    wrapped_vec.iter().for_each(|v| println!("{}", v));
    wrapped_vec.into_iter().for_each(|v| println!("{}", v));

    let forward = vec![1, 2, 3];
    let reversed = forward.reversed();
    dbg!(&forward);
    dbg!(&reversed);

    let other_reversed = reversed.iter().to_reversed();
    dbg!(&other_reversed);
}

struct WrappedVec<T>(Vec<T>);

impl<T> Deref for WrappedVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> WrappedVec<T> {
    fn into_iter(self) -> IntoIter<T> {
        self.0.into_iter()
    }
}

pub trait ReverseExt<T> {
    fn reversed(&self) -> Vec<T>;
}

impl<T> ReverseExt<T> for Vec<T>
where
    T: Clone,
{
    fn reversed(&self) -> Vec<T> {
        self.iter().rev().cloned().collect()
    }
}

pub trait DoubleEndedIteratorExt: DoubleEndedIterator {
    fn to_reversed<'a, T>(self) -> Vec<T>
    where
        T: 'a + Clone,
        Self: Sized + Iterator<Item = &'a T>;
}

impl<I: DoubleEndedIterator> DoubleEndedIteratorExt for I {
    fn to_reversed<'a, T>(self) -> Vec<T>
    where
        T: 'a + Clone,
        Self: Sized + Iterator<Item = &'a T>,
    {
        self.rev().cloned().collect()
    }
}

fn get_middle(s:&str) -> &str {
    let len = s.len();
    if len % 2 == 0 {
        &s[len/2-1..len/2+1]
    } else {
        &s[len/2..(len/2)+1]
    }
}

fn sum_two_smallest_numbers(numbers: &[u32]) -> u32 {
    //let mut sorted = numbers.to_owned();
    // let mut numbers = numbers.to_vec();
    // numbers.sort();
    // numbers[..2].iter().sum()

    let mut lowest1 = numbers[0]; 
    let mut lowest2 = if numbers[1] > lowest1 { 
        numbers[1] 
    } else {
        lowest1 = numbers[1]; 
        numbers[0] 
    };

    for &i in numbers[2..].iter() {
        if i < lowest1 {
            lowest2 = lowest1;
            lowest1 = i;
        } else if i < lowest2 {
            lowest2 = i;
        }
    }
    lowest1 + lowest2
}

fn string_to_number(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

fn disemvowel(s: &str) -> String {
    let mut ret = String::new();
    for i in s.chars() {
        // match i {
        //     'A' | 'a' => {}
        //     'E' | 'e' => {}
        //     'I' | 'i' => {}
        //     'O' | 'o' => {}
        //     'U' | 'u' => {}
        //     _ => { ret.push((i)) },
        // }
        // if matches!(i, 'A' | 'a' | 'E' | 'e' | 'I' | 'i' | 'O' | 'o' | 'U' | 'u') {  
        //     // Do nothing  
        // } else {  
        //     ret.push(i);  
        // } 
        if matches!(i, 'A' | 'a' | 'E' | 'e' | 'I' | 'i' | 'O' | 'o' | 'U' | 'u') == false {
            ret.push(i);
        }
    }
    ret
}

fn positive_sum(slice: &[i32]) -> i32 {
    //slice.iter().filter(|&&x| x > 0).sum()
    let mut ret = 0;
    for &i in slice {
        if i > 0 {
            ret += i;
        }
    }
    ret
}

fn rps(p1: &str, p2: &str) -> &'static str  {
    if p1 == p2 { return "Draw!"; }

    let p1p2 = (p1, p2);
    match p1p2 {
        ("scissors", "paper") => "Player 1 won!",
        ("rock", "scissors") => "Player 1 won!",
        ("paper", "rock") => "Player 1 won!",
        _ => "Player 2 won!"
    }
}

fn greet(name: &str) -> String {
    format!("Hello, {} how are you doing today?", name)
}

struct Buffer0 {
    buf: [u8; 256],
}

struct Buffer00<T> {
    buf: [T; 256],
}

#[derive(Debug)]
struct Buffer<T, const LENGTH: usize> {
    buf: [T; LENGTH],
}

// impl From<[u8; 256]> for Buffer<u8, 256> {
//     fn from(buf: [u8; 256]) -> Self {
//         Buffer { buf }
//     }
// }

impl<T, const LENGTH: usize> From<[T; LENGTH]> for Buffer<T, LENGTH> {
    fn from(buf: [T; LENGTH]) -> Self {
        Buffer { buf }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(greet("Ryan"), "Hello, Ryan how are you doing today?");
        assert_eq!(
            greet("Shingles"),
            "Hello, Shingles how are you doing today?"
        );
    }

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(p1: &str, p2: &str, expected: &str) {
        assert_eq!(rps(p1, p2), expected, "{ERR_MSG} with p1 = \"{p1}\", p2 = \"{p2}\"")   
    }

    #[test]
    fn fixed_tests() {
        dotest("rock", "scissors", "Player 1 won!");
        dotest("scissors", "rock", "Player 2 won!");
        dotest("rock", "rock", "Draw!");
    }

    #[test]
    fn some_examples() {
        assert_eq!(positive_sum(&[1,2,3,4,5]), 15);
        assert_eq!(positive_sum(&[1,-2,3,4,5]), 13);
        assert_eq!(positive_sum(&[-1,2,3,4,-5]), 9);
    }
    
    #[test]
    fn empty_list() {
        assert_eq!(positive_sum(&[]), 0);
    }
    
    #[test]
    fn all_negative() {
        assert_eq!(positive_sum(&[-1,-2,-3,-4,-5]), 0);
    }

    #[test]
    fn example_test() {
        assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
    }

    #[test]
    fn returns_expected() {
      assert_eq!(string_to_number("1234"), 1234);
      assert_eq!(string_to_number("605"), 605);
      assert_eq!(string_to_number("1405"), 1405);
      assert_eq!(string_to_number("-7"), -7);
    }

    #[test]
    fn sum_two_smallest_numbers_tests() {
        assert_eq!(sum_two_smallest_numbers(&[5, 8, 12, 19, 22]),  13, "Incorrect result for [5, 8, 12, 19, 22]");
        assert_eq!(sum_two_smallest_numbers(&[15, 28, 4, 2, 43]), 6, "Incorrect result for [15, 28, 4, 2, 43]");
        assert_eq!(sum_two_smallest_numbers(&[23, 71, 33, 82, 1]), 24, "Incorrect result for [23, 71, 33, 82, 1]");
        assert_eq!(sum_two_smallest_numbers(&[52, 76, 14, 12, 4]), 16, "Incorrect result for [52, 76, 14, 12, 4]");
        assert_eq!(sum_two_smallest_numbers(&[1, 1, 5, 5]),  2, "Incorrect result for [1, 1, 5, 5]");
    }

    #[test]
    fn example_get_middle_tests() {
        assert_eq!(get_middle("test"),"es");
        assert_eq!(get_middle("testing"),"t");
        assert_eq!(get_middle("middle"),"dd");
        assert_eq!(get_middle("A"),"A");
        assert_eq!(get_middle("of"),"of");
    }
}
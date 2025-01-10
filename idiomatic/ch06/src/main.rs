fn main() {
    println!("{}", make_upper_case("hello"));
    println!("{}", points(&[String::from("1:0")]));
    println!("{}", points(&[String::from("1:1")]));
    println!("{}", points(&[String::from("0:2")]));
    println!("{}", reverse_string_solution("buru buru"));
    println!("{}", &find_smallest_int(&[34, -345, -1, 100]));

    let x = Some(5);
    match x {
        Some(v) => println!("Value: {}", v), // Moves the value out
        None => println!("No value"),
    }

    println!("Value: {}", x.unwrap());

    let x = Some(5);
    match &x {
        Some(v) => println!("Value: {}", v), // Moves the value out
        None => println!("No value"),
    }

    println!("Value: {}", x.unwrap());
    //println!("Value: {}", x); x was moved

    let x = 5;
    let y = &x; // `y` is a reference to `x`
    println!("x: {}, y: {}", &x, &y);
    println!("x: {}, y: {}", x, y);
    println!("x: {}, y: {}", x, *y);

    let pair = (1, 2);

    match pair {
        (x, y) => {
            println!("x: {}, y: {}", x, y); // `x` is owned, `y` is a reference
        }
    }

    println!("x: {}, y: {}", pair.0, pair.1);

    let point = Point { x: 5, y: 10 };

    match point {
        Point { x, y } => {
            // `x` is taken by value, `y` is borrowed
            println!("x: {}, y: {}", x, y);
        }
    }

    println!("x: {}, y: {}", point.x, point.y);

    let greetings: Option<String> = Some(String::from("Hello, World!"));

    // Using `ref` to borrow the value inside the Option
    match greetings {
        Some(ref msg) => println!("{}", msg), // This works: msg is a reference to the inner String
        None => println!("Nothing to greet"),
    }

    // Correctly accessing `greetings` again after using `ref`
    println!("Greetings? {:?}", greetings.unwrap()); // This works because we didn't move the value
    //println!("Greetings? {:?}", greetings); // This works because we didn't move the value

    let greetings: Option<u64> = Some(123);
    match greetings {
        Some(msg) => println!("{}", msg),
        None => println!("Nothing to greet"),
    }
    println!("Greetings? {:?}", greetings.unwrap());
}

struct Point {
    x: i32,
    y: i32,
}

fn find_smallest_int(arr: &[i32]) -> i32 {
    let mut min:&i32 = &arr[0];
    for i in arr {
        if i < min {
            min = i;
        }
    }

    *min
}

fn boolean_to_string(b: bool) -> String {
    b.to_string()
}

fn reverse_string_solution(phrase: &str) -> String {
    let mut reversed = String::new();
    for c in phrase.chars().rev() {
        reversed.push(c);
    }

    reversed
}

fn points(games: &[String]) -> u32 {
    let mut finalResult = 0;
    for game in games {
        let homeTeam: u8 = game[0..1].parse().unwrap(); //game.chars().nth(0).unwrap() as u8;// 
        let awayTeam = game[2..3].parse().unwrap(); //game.chars().nth(2).unwrap() as u8;// 

        if homeTeam > awayTeam {
            finalResult += 3;
        } else if homeTeam == awayTeam {
            finalResult += 1;
        }
    }
    finalResult
}

fn make_upper_case(s: &str) -> String {
    let mut upper_str = String::new();
    for c in s.chars() {
        match c as u8 {
            97 ..= 122 => upper_str.push( ((c as u8) - 32) as char),
            _ => upper_str.push(c)
        }
    }
    //s.to_uppercase()
    upper_str
}

#[cfg(test)]
mod tests {
    // use super::points;
    // use super::make_upper_case;
    use super::*;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn do_fixed_test(e: &[&str], expected: u32) {
        let a = &e.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        assert_eq!(points(a), expected, "{ERR_MSG} with games = {a:?}")
    }

    #[test]
    fn fixed_tests() {
        do_fixed_test(&["1:0", "2:0", "3:0", "4:0", "2:1", "3:1", "4:1", "3:2", "4:2", "4:3"], 30);
        do_fixed_test(&["1:1", "2:2", "3:3", "4:4", "2:2", "3:3", "4:4", "3:3", "4:4", "4:4"], 10);
        do_fixed_test(&["0:1", "0:2", "0:3", "0:4", "1:2", "1:3", "1:4", "2:3", "2:4", "3:4"], 0);
        do_fixed_test(&["1:0", "2:0", "3:0", "4:0", "2:1", "1:3", "1:4", "2:3", "2:4", "3:4"], 15);
        do_fixed_test(&["1:0", "2:0", "3:0", "4:4", "2:2", "3:3", "1:4", "2:3", "2:4", "3:4"], 12);
    }

    #[test]
    fn test_make_upper_case() {
        assert_eq!(make_upper_case("hello"), "HELLO");
    }

    #[test]
    fn reverse_string_solution_test() {
        assert_eq!(reverse_string_solution("world"), "dlrow");
    }

    #[test]
    fn example() {
        assert_eq!(boolean_to_string(true), "true", "When we pass in true, we want the string \"true\" as output");
        assert_eq!(boolean_to_string(false), "false", "When we pass in false, we want the string \"false\" as output");
        assert_eq!(boolean_to_string(false), "false", "When we pass in false, we want the string \"false\" as output");
    }

    #[test]
    fn sample_tests() {
        assert_eq!(find_smallest_int(&[34, 15, 88, 2]), 2);
        assert_eq!(find_smallest_int(&[34, -345, -1, 100]), -345);
    }
}




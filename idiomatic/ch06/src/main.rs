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
    println!("square_sum = {:?}", square_sum(vec![1, 2]));
    println!("{}", are_you_playing_banjo("rartin"));
}

/// Provides a singly linked list implementation with iterator access.
pub struct LinkedList<T> {
    head: Option<ListItemPtr<T>>,
}

impl<T> LinkedList<T> {
    /// Constructs a new, empty [`LinkedList<T>`].
    pub fn new() -> Self {
        Self { head: None }
    }
    /// Appends an element to the end of the list. If the list is empty,
    /// the element becomes the first element of the list.
    pub fn append(&mut self, t: T) {
        match &self.head {
            Some(head) => {
                let mut next = head.clone();
                while next.as_ref().borrow().next.is_some() {
                    let n = next.as_ref()
                        .borrow().next.as_ref().unwrap().clone();
                    next = n;
                }
                next.as_ref().borrow_mut().next = Some(Rc::new(RefCell::new(ListItem::new(t))));
            }
            None => {
                self.head = Some(Rc::new(RefCell::new(ListItem::new(t))));
            }
        }
    }
    /// Returns an iterator over the list.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.clone(),
            data: None,
            phantom: PhantomData,
        }
    }
    /// Returns an iterator over the list that allows mutation.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.clone(),
            data: None,
            phantom: PhantomData,
        }
    }
    /// Consumes this list returning an iterator over its values.
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            next: self.head.clone(),
        }
    }
}

fn are_you_playing_banjo(name: &str) -> String {
    match &name[0..1] {
        "R" | "r" => format!("{} plays banjo", name),
        _ => format!("{} does not play banjo", name)
    }
}

fn are_you_playing_banjo2(name: &str) -> String {
    let char = name.chars().nth(0).unwrap();
    let mut nameString = String::from(name);
    
    match char {
        'R' => nameString.push_str(" plays banjo"),
        'r' => nameString.push_str(" plays banjo"),
        _ => nameString.push_str(" does not play banjo"),
    };

    nameString
}

fn bool_to_word(value: bool) -> &'static str {
    let ret = match value {
        true => "Yes",
        false => "No"
    };
    ret
}

fn square_sum(vec: Vec<i32>) -> i32 {
    let mut finalResult = 0;
    for i in vec {
        finalResult += i*i;
    }

    finalResult
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

        #[test]
    fn returns_expected() {
        assert_eq!(square_sum(vec![1, 2]), 5);
        assert_eq!(square_sum(vec![-1, -2]), 5);
        assert_eq!(square_sum(vec![5, 3, 4]), 50);
        assert_eq!(square_sum(vec![]), 0);
    }

    #[test]
    fn example_tests() {
        assert_eq!(bool_to_word(true), "Yes");
        assert_eq!(bool_to_word(false), "No");
    }

    #[test]
    fn test_are_you_playing_banjo() {
        assert_eq!(are_you_playing_banjo("Martin"), "Martin does not play banjo");
        assert_eq!(are_you_playing_banjo("Rikke"), "Rikke plays banjo");
        assert_eq!(are_you_playing_banjo("bravo"), "bravo does not play banjo");
        assert_eq!(are_you_playing_banjo("rolf"), "rolf plays banjo");
    }
}




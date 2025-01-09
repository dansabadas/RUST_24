fn main() {
    println!("{}", make_upper_case("hello"));
    println!("{}", points(&[String::from("1:0")]));
    println!("{}", points(&[String::from("1:1")]));
    println!("{}", points(&[String::from("0:2")]));
    println!("{}", reverse_string_solution("buru buru"));
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
}




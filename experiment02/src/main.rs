use experiment02::{Inventory, ShirtColor};
fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

fn number_of_carries(a: u32, b: u32) -> usize {
    2
}

#[cfg(test)]
mod tests {
    use super::number_of_carries;

    #[test]
    fn basic_tests() {
        do_test(543, 3456, 0);
        do_test(1927, 6426, 2);
        do_test(9999, 1, 4);
        do_test(1234, 5678, 2);
    }
    
    fn do_test(a: u32, b: u32, exp: usize) {
        let user_result = number_of_carries(a, b); 
        assert!(
            user_result == exp,
            "Expected number_of_carries({0}, {1}) to equal {2}\nInstead, number_of_carries({0}, {1}) was {3}",
            a,
            b,
            exp,
            user_result
        )
    }
}
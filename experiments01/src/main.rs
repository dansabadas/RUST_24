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

#[cfg(test)]
mod tests {
    use super::*;

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
}
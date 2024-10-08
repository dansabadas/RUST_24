fn main() {
	let seasons = ["Spring", "Summer", "Autumn", "Winter"];
	let seasons2 = ["Spring", "Summer", "Fall", "Autumn", "Winter"];
	//seasons.ddd();
	//seasons2.thd();

	let my_array = ["a"; 5];
    println!("{:?}", my_array);
    println!("{:?}", seasons2);

    let my_numbers = [0, 10, -20];
    println!("{}", my_numbers[1]);
    let array_of_ten = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let two_to_five = &array_of_ten[2..5];
    let start_at_one = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];
    //array_of_ten.thd();
    //end_at_five.thd();

    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");

    //let mut my_vec1 = Vec::new();//fails compilation if we don't add a object with type
    let mut my_vec: Vec<String> = Vec::new();
    my_vec.push(name1);
    my_vec.push(name2);

    let mut num_vec = Vec::new();
    println!("Capacitor: {}", num_vec.capacity());
    num_vec.push('a');
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    num_vec.push('a');
    num_vec.push('a');
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    println!("{}", num_vec.capacity());

    let my_vec: Vec<u8> = [1, 2, 3].into();
    let my_vec2: Vec<_> = [9, 0, 10].into();
    //my_vec2.tbh();
    let strings = (
        "one".to_string(),
        "two".to_string(),
        "three".to_string()
    );
    let (a, b, c) = strings;
    println!("{b}");

    let tuple_of_three = ("one", "two", "three");
    let (_, b, c) = tuple_of_three;

    let my_number = 5;
    if my_number == 7 {
        println!("It's seven");
    } else if my_number == 6 {
        println!("It's six")
    } else {
        println!("It's a different number")
    }

    let my_number: u8 = 5;
    match my_number {
        0 => println!("it's zero"),
        1 => println!("it's one"),
        2 => println!("it's two"),
        _ => println!("It's some other number"),
    }

    let my_number = 5;
    let second_number = match my_number {
        0 => 0,
        5 => 10,
        _ => 2,
    };
    let sky = "cloudy";
    let temperature = "warm";
    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's dark and unpleasant today"),
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", "warm") => println!("It's dark but not bad"),
        _ => println!("Not sure what the weather is."),
    }

    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);
    let fourth = (0, 50, 0);
    match_colors(first);
    match_colors(second);
    match_colors(third);
    match_colors(fourth);

    let mut counter = 0;
    let mut counter2 = 0;
    println!("Now entering the first loop.");
    'first_loop: loop {
        counter += 1;
        println!("The counter is now: {}", counter);
        if counter > 5 {
            println!("Now entering the second loop.");
            'second_loop: loop {
                println!("The second counter is now: {}", counter2);
                counter2 += 1;
                if counter2 == 3 {
                    break 'first_loop;
                }
            }
        }
    }
    for number in 0..=3 {
        println!("The next number is: {}", number);
    }
    for _ in 0..3 {
        println!("Printing the same thing three times");
    }

    let mut counter = 5;
    let my_number = loop {
        counter +=1;
        if counter % 53 == 3 {
            break counter;
        }
    };
    println!("{my_number}");

    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);
    match_colors2(first);
    match_colors2(second);
    match_colors2(third);
}

fn match_colors2(rbg: (i32, i32, i32)) {
    let (red, blue, green) = (rbg.0, rbg.1, rbg.2);
    println!("Comparing a color with {red} red, {blue} blue, and
    {green} green:");
    let color_vec = vec![(red, "red"), (blue, "blue"),
    (green, "green")];
    let mut all_have_at_least_10 = true;
    for (amount, color) in color_vec {
        if amount < 10 {
            all_have_at_least_10 = false;
            println!("Not much {color}.");
        }
    }
    if all_have_at_least_10 {
        println!("Each color has at least 10.")
    }
    println!();
}

fn match_colors(rgb: (i32, i32, i32)) {
    match rgb {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, g, _) if g < 10 => println!("Not much green"),
        (_, _, b) if b < 10 => println!("Not much blue"),
        _ => println!("Each color has at least 10"),
    }
}
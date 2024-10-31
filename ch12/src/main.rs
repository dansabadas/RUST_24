use std::{fmt::Display, sync::{Arc, Mutex}};
use std::thread::spawn;

fn main() {
    let some_vec = vec![9, 8, 10];
    do_something(|| {
        some_vec
        .into_iter()
        .for_each(|x| println!("The number is: {x}"));
    });

    let some_vec = vec![9, 8, 10];
    do_something2(|| {
        some_vec.iter().for_each(|x| println!("The number is: {x}"));
    });
    do_something2(|| {
        some_vec.iter().for_each(|x| println!("The number is: {x}"));
    });

    let mut my_string = String::from("Hello there");
    let prints_string = || {
        println!("{my_string}");
    };
    takes_fnonce(prints_string);
    //takes_fn(prints_string);
    let adds_exclamation_and_prints = || {
        my_string.push('!');
        println!("{my_string}");
    };
    takes_fnmut(adds_exclamation_and_prints);
    let prints_then_drops = || {
        println!("Now dropping {my_string}");
        drop(my_string);
    };
    takes_fnonce(prints_then_drops);
    //takes_fnonce(prints_then_drops);

    let my_closure = || 9;
    takes_a_closure_and_does_nothing(my_closure);

    let first_closure = || 9;
    let second_closure = || 9;
    //takes_two_closures_and_does_nothing(first_closure, second_closure);
    takes_two_closures_and_does_nothing(first_closure, first_closure);
    takes_two_closures_and_does_nothing2(first_closure, second_closure);

    let mut tallinn = City {
        name: "Tallinn".to_string(),
        years: vec![1372, 1834, 1897, 1925, 1959, 1989, 2000, 2010, 2020],
        populations: vec![3_250, 15_300, 58_800, 119_800, 283_071, 478_974, 400_378, 406_703, 437_619],
    };

    tallinn.change_city_data(|x, y| {
        x.push(2030);
        y.push(500_000);
    });

    tallinn.change_city_data(|years, populations| {
        let new_vec = years
            .iter_mut()
            .zip(populations.iter_mut())
            .take(3)
            .collect::<Vec<(_, _)>>();
        println!("{new_vec:?}");
    });

    tallinn.change_city_data(|x, y| {
        let position_option = x.iter().position(|x| *x == 1834);
        if let Some(position) = position_option {
            println!(
                "Going to delete {} at position {:?} now.",
                x[position], position
            );
            x.remove(position);
            y.remove(position);
        }
    });

    println!(
        "Years left are {:?}\nPopulations left are {:?}",
        tallinn.years, tallinn.populations
    );

    print_maximum(8, 10);
    print_maximum2(8, 10);

    let name = "Tuon";
    let string_name = String::from("Tuon");
    prints_it(name);
    prints_it(string_name);

    prints_it_regular_generic::<u8>(100);
    prints_it_impl_trait(100);
    prints_it_impl_trait(100u8);
    // prints_it_impl_trait::<u8>(100);

    let my_number = 10;
    let mut doubles = returns_a_closure("double");
    let mut triples = returns_a_closure("triple");
    let mut does_nothing = returns_a_closure("HI");
    let doubled = doubles(my_number);
    let tripled = triples(my_number);
    let same = does_nothing(my_number);

    let handle = std::thread::spawn(|| {
        println!("The thread is working!")
    });
    handle.join().unwrap();

    let handle = std::thread::spawn(|| {
        for _ in 0..5 {
            println!("The thread is working!")
        }
    });
    handle.join().unwrap();

    let thread1 = std::thread::spawn(|| {
        for _ in 0..5 {
            println!("Thread 1 is working!")
        }
    });
    let thread2 = std::thread::spawn(|| {
        for _ in 0..5 {
            println!("Thread 2 is working!")
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    let my_number = Arc::new(Mutex::new(0));
    let cloned_1 = Arc::clone(&my_number);
    let cloned_2 = Arc::clone(&my_number);
    let thread1 = std::thread::spawn(move || {
        for _ in 0..10 {
            *cloned_1.lock().unwrap() += 1;
        }
    });
    let thread2 = std::thread::spawn(move || {
        for _ in 0..10 {
            *cloned_2.lock().unwrap() += 1;
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("Value is: {my_number:?}");

    let my_number = Arc::new(Mutex::new(0));
    let mut handle_vec = vec![];
    for _ in 0..10 {
        let my_number_clone = Arc::clone(&my_number);
        let handle = std::thread::spawn(move || {
        for _ in 0..10 {
            *my_number_clone.lock().unwrap() += 1;
        }
        });
        handle_vec.push(handle);
    }
    handle_vec.into_iter().for_each(|handle| handle.join().unwrap());
    println!("{my_number:?}");

    let mut handle_vec = vec![];
    let my_number = make_arc(0);
    for _ in 0..10 {
        let my_number_clone = new_clone(&my_number);
        let handle = spawn(move || {
            for _ in 0..10 {
                let mut value_inside = my_number_clone.lock().unwrap();
                *value_inside += 1;
            }
        });
        handle_vec.push(handle);
    }
    handle_vec.into_iter().for_each(|handle| handle.join().unwrap());
    println!("{my_number:?}");

    println!("Exiting the program");
}

fn make_arc(number: i32) -> Arc<Mutex<i32>> {
    Arc::new(Mutex::new(number))
}

fn new_clone(input: &Arc<Mutex<i32>>) -> Arc<Mutex<i32>> {
    Arc::clone(&input)
}

fn returns_a_closure(input: &str) -> impl FnMut(i32) -> i32 {
    match input {
        "double" => |mut number| {
            number *= 2;
            println!("Doubling number. Now it is {number}");
            number
        },
        "triple" => |mut number| {
            number *= 3;
            println!("Tripling number. Now it is {number}");
            number
        },
        _ => |number| {
            println!("Sorry, it's the same: {number}.");
            number
        },
    }
}
fn gives_higher2(one: impl PartialOrd + Display, two: impl PartialOrd + Display) {
    // let higher = if one > two { one } else { two };
    // println!("{higher} is higher.");
}

fn prints_it_impl_trait(input: impl Display) {
    println!("You can print many things, including {input}");
}
fn prints_it_regular_generic<T: Display>(input: T) {
    println!("You can print many things, including {input}");
}

fn prints_it(input: impl Into<String> + Display) {
    println!("You can print many things, including {input}");
}

fn print_maximum2<T: PartialOrd + Display>(one: T, two: T) {
    let higher = if one > two { one } else { two };
    println!("{higher} is higher.");
}

fn print_maximum(one: i32, two: i32) {
    let higher = if one > two { one } else { two };
    println!("{higher} is higher");
}

#[derive(Debug)]
struct City {
    name: String,
    years: Vec<u32>,
    populations: Vec<u32>,
}

impl City {
    fn change_city_data<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Vec<u32>, &mut Vec<u32>),
    {
        f(&mut self.years, &mut self.populations)
    }
}

fn takes_two_closures_and_does_nothing2<F, G>(first: F, second: G)
where
    F: Fn() -> i32,
    G: Fn() -> i32,
{
}

fn takes_two_closures_and_does_nothing<F>(first: F, second: F)
where
    F: Fn() -> i32,
{
}

fn takes_a_closure_and_does_nothing<F>(f: F)
where
    F: Fn() -> i32,
{}

fn takes_fnonce<F: FnOnce()>(f: F) {
    f();
}
fn takes_fnmut<F: FnMut()>(mut f: F) {
    f();
    f();
}

fn takes_fn<F: Fn()>(f: F) {
    f();
    f();
}

fn do_something2<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn do_something<F>(f: F)
where
    F: FnOnce(),
{
    f();
}
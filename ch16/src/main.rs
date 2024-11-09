use std::sync::Mutex;
use std::mem::transmute;
use rand::prelude::*;
use rand::{thread_rng, Rng};

fn main() {
    let buffer_1 = Buffers {
        array_one: [0u8; 3],
        array_two: [0; 3],
    };
    let buffer_2 = Buffers {
        array_one: [0i32; 4],
        array_two: [10; 4],
    };

    println!("{buffer_1:#?}, {buffer_2:#?}");

    let mut my_vec = Vec::new();
    my_vec.push(give_eight());
    my_vec.push(NUMBER);

    println!("{my_vec:#?}");

    add_message("2022-12-12");
    add_message("2023-05-05");
    println!("{GLOBAL_LOGGER:#?}");

    let my_name = unsafe { "My name" };
    println!("{my_name}");

    unsafe {
        uh_oh();
    }

    //NUMBER += 1;
    //println!("{NUMBER2}");

    let mut join_handle_vec = vec![];
    for _ in 0..1000 {
        join_handle_vec.push(std::thread::spawn(|| {
        for _ in 0..1000 {
            unsafe {
                NUMBER2 += 1;
            }
        }
        }));
    }

    for handle in join_handle_vec {
        handle.join().unwrap();
    }

    unsafe {
        println!("{NUMBER2}");
    }

    let x = -19;
    let y = unsafe { transmute::<i32, u32>(x) };
    println!("{y} {y:b}");

    println!("{}", std::mem::size_of::<User>());

    let some_i32s = [1, 2, 3, 4, 5, 6, 7, 8];
    //let user = unsafe { transmute::<[i32; 8], User>(some_i32s) };

    let my_option = Some(10);
    // SAFETY: my_option is declared as Some(10). It will never be None
    let unwrapped = unsafe {
        my_option.unwrap_unchecked()
    };
    println!("{unwrapped}");

    if rand::random() { // generates a boolean
        // Try printing a random unicode code point (probably a bad idea)!
        println!("char: {}", rand::random::<char>());
    }

    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen(); // generates a float between 0 and 1

    let mut nums: Vec<i32> = (1..100).collect();
    nums.shuffle(&mut rng);

    for _ in 0..5 {
    let random_u16 = random::<u16>();
        print!("{random_u16} ");
    }

    let mut number_maker = thread_rng();
    for _ in 0..5 {
        print!("{} ", number_maker.gen_range(1..11));
    }

    let weak_billy = Character::new(Dice::Three);
    let strong_billy = Character::new(Dice::Four);
    println!("{weak_billy:#?}");
    println!("{strong_billy:#?}");
}

#[derive(Debug)]
struct Character {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
}

#[derive(Copy, Clone)]
enum Dice {
    Three,
    Four,
}

fn roll_dice(dice_choice: Dice) -> u8 {
    let mut generator = thread_rng();
    let mut total = 0;
    match dice_choice {
        Dice::Three => {
            for _ in 0..3 {
                total += generator.gen_range(1..=6);
            }
        }
        Dice::Four => {
            let mut results = vec![];
            (0..4).for_each(|_| results.push(generator.gen_range(1..=6)));
            results.sort();
            results.remove(0);
            total += results.into_iter().sum::<u8>();
        }
    }
    total
}

impl Character {
    fn new(dice_choice: Dice) -> Self {
        let mut stats = (0..6).map(|_| roll_dice(dice_choice));
        Self {
            strength: stats.next().unwrap(),
            dexterity: stats.next().unwrap(),
            constitution: stats.next().unwrap(),
            intelligence: stats.next().unwrap(),
            wisdom: stats.next().unwrap(),
            charisma: stats.next().unwrap(),
        }
    }
}

struct User {
    name: String,
    number: u32,
}

static mut NUMBER2: u32 = 0;

unsafe fn uh_oh() {}

#[derive(Debug)]
struct Log {
    date: &'static str,
    message: String,
}

static GLOBAL_LOGGER: Mutex<Vec<Log>> = Mutex::new(Vec::new());

fn add_message(date: &'static str) {
    GLOBAL_LOGGER.lock().unwrap().push(Log {
        date,
        message: "Everything's fine".to_string(),
    });
}

const NUMBER: u8 = give_eight();

const fn give_eight() -> u8 {
    8
}

#[derive(Debug)]
struct Buffers<T, const N: usize> {
    array_one: [T; N],
    array_two: [T; N]
}
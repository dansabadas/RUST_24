use std::sync::Mutex;
use std::mem::transmute;

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
    let user = unsafe { transmute::<[i32; 8], User>(some_i32s) };
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
use std::fmt::Display;
use std::cell::Cell;
use std::cell::RefCell;
use std::sync::Mutex;
use std::sync::RwLock;

fn prints_str(my_str: &str) {
    println!("{my_str}");
}

fn works() -> &'static str {
    "I live forever!"
}

fn returns_str() -> &'static str {
    let my_string = String::from("I am a string");
    "I am a str"
}

fn main() {
    let my_string = String::from("I am a string");
    prints_str(&my_string);
    //prints_str(my_string);

    let my_str = returns_str();
    println!("{my_str}");

    let my_city = City {
        name: "Ichinomiya",
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded);

    let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];
    let my_city = City2 {
        name: &city_names[0],
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded);

    let mut billy = Adventurer {
        name: "Billy",
        hit_points: 100_000,
    };
    println!("{}", billy);
    billy.take_damage();

    let super_phone_3000 = PhoneModel {
        company_name: "YY Electronics".to_string(),
        model_name: "Super Phone 3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_000,
        date_issued: 2020,
        on_sale: Cell::new(true),// true,
    };
    super_phone_3000.make_not_on_sale();
    println!("{super_phone_3000:#?}");

    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };
    println!("{:?}", user_1.active);

    //let mut borrow = user_1.active.borrow_mut();
    //*borrow = false;
    *user_1.active.borrow_mut() = false;
    println!("{:?}", user_1.active);
    *user_1.active.borrow_mut() = true;
    println!("{:?}", user_1.active);

    // let bool_in_refcell = RefCell::new(true);
    // std::thread::spawn(|| {
    //     *bool_in_refcell.borrow_mut() = false;
    // });

    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap();
    println!("{my_mutex:?}");
    println!("{mutex_changer:?}");
    *mutex_changer = 6;
    println!("{mutex_changer:?}");

    let my_mutex = Mutex::new(5);
    {
        let mut mutex_changer = my_mutex.lock().unwrap();
        *mutex_changer = 6;
    }
    println!("{my_mutex:?}");

    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap();
    *mutex_changer = 6;
    drop(mutex_changer);
    println!("{my_mutex:?}");

    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap();
    let mut other_mutex_changer = my_mutex.try_lock();
    if let Ok(value) = other_mutex_changer {
        println!("The MutexGuard has: {value}")
    } else {
        println!("Didn't get the lock")
    }
    let my_mutex = Mutex::new(5);
    *my_mutex.lock().unwrap() = 6;
    println!("The MutexGuard has: {my_mutex:?}");
    let my_mutex = Mutex::new(5);
    for _ in 0..100 {
        *my_mutex.lock().unwrap() += 1;
    }
    println!("The MutexGuard has: {my_mutex:?}");

    let my_rwlock = RwLock::new(5);
    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();
    println!("{read1:?}, {read2:?}");
    //let write1 = my_rwlock.write().unwrap();
    drop(read1);
    drop(read2);
    let mut write1 = my_rwlock.write().unwrap();
    *write1 = 6;
    drop(write1);
    println!("{:?}", my_rwlock);

    let my_rwlock = RwLock::new(5);
    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();
    if let Ok(mut number) = my_rwlock.try_write() {
        *number += 10;
    println!("Now the number is {}", number);
    } else {
        println!("Couldn't get write access, sorry!")
    };
}

#[derive(Debug)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>,
}

#[derive(Debug)]
struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: Cell<bool>,
}

impl PhoneModel {
    fn method_one(&self) {}
    fn method_two(&self) {}
    fn make_not_on_sale(&self) {
        self.on_sale.set(false);
    }
}

struct Adventurer<'a> {
    name: &'a str,
    hit_points: u32,
}

impl Adventurer<'_> {
    fn take_damage(&mut self) {
        self.hit_points -= 20;
        println!("{} has {} hit points left!", self.name, self.hit_points);
    }
}

impl std::fmt::Display for Adventurer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} has {} hit points.", self.name, self.hit_points)
    }
}

fn prints<T: Display>(input: T) {
    println!("T is {input}");
}

#[derive(Debug)]
struct City3<'city> {
    name: &'city str,
    date_founded: u32,
}

#[derive(Debug)]
struct City2<'a> {
    name: &'a str,
    date_founded: u32,
}

#[derive(Debug)]
struct City {
    name: &'static str,
    date_founded: u32,
}
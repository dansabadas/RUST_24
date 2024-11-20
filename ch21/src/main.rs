use std::mem;
use std::time::Duration;

fn main() {
    println!("Size of an i32: {}", mem::size_of::<i32>());
    let my_array = [8; 50];
    println!("Size of this array: {}", mem::size_of_val(&my_array));
    let some_string = String::from("Droppable because it's not Copy");
    println!("Size of this str: {}", mem::size_of_val(&some_string));
    drop(some_string);
    // some_string.clear();

    let mut one_ring = Ring {
        owner: "Frodo".into(),
        former_owners: vec!["Gollum".into(), "Sauron".into()],
    };
    println!("Original state: {one_ring:?}");
    one_ring.switch_owner_to("Gollum");
    println!("{one_ring:?}");
    one_ring.switch_owner_to("Sauron");
    println!("{one_ring:?}");
    one_ring.switch_owner_to("Billy");
    println!("{one_ring:?}");

    let mut capital_city = City {
        name: "Constantinople".to_string(),
    };
    capital_city.change_name("Istanbul");

    let mut number_vec = vec![8, 7, 0, 2, 49, 9999];
    let mut new_vec = vec![];
    number_vec.iter_mut().for_each(|number| {
        let taker = mem::take(number);
        new_vec.push(taker);
    });
    println!("{:?}\n{:?}", number_vec, new_vec);

    let mut bank_of_klezkavania = Bank {
        money_inside: 5000,
        money_at_desk: DeskMoney(500),
    };
    let money_stolen = mem::take(&mut bank_of_klezkavania.money_at_desk);
    println!("Stole {} Klezkavanian credits", money_stolen.0);
    println!("{bank_of_klezkavania:?}");

    let mut user_state = UserState {
        username: "Mr. User".to_string(),
        connection: None,
    };
    user_state.connect("someurl.com");
    println!("Connected? {}", user_state.is_connected());
    user_state.disconnect();
    println!("Connected? {}", user_state.is_connected());

    //panic!();
    //panic!("Oh man, something went wrong");
    let try_parse = "my_num".parse::<u32>();
    println!("Error output: {try_parse:?}");
    //let my_num = try_parse.unwrap();
    std::panic::set_hook(Box::new(|_| {
        println!("Oops, that didn't work.");
        println!("앗 뭔가 잘못 됐네요.");
    }));
    std::panic::set_hook(Box::new(|info| {
        println!("Well, that didn't work: {info}");
    }));
    std::panic::set_hook(Box::new(|info| {
        if let Some(payload) = info.payload().downcast_ref::<&str>() {
            println!("{payload}");
        } else {
            println!("No payload!");
        }
    }));
    panic!("Oh no");
    panic!();
}

struct UserState {
    username: String,
    connection: Option<Connection>,
}
struct Connection {
    url: String,
    timeout: Duration,
}
impl UserState {
    fn is_connected(&self) -> bool {
        self.connection.is_some()
    }
    fn connect(&mut self, url: &str) {
        self.connection = Some(Connection {
            url: url.to_string(),
            timeout: Duration::from_secs(3600),
        });
    }
    fn disconnect(&mut self) {
        self.connection.take();
    }
}

#[derive(Debug)]
struct Bank {
    money_inside: u32,
    money_at_desk: DeskMoney,
}

#[derive(Debug)]
struct DeskMoney(u32);
impl Default for DeskMoney {
    fn default() -> Self {
        Self(50)
    }
}

struct City {
    name: String,
}
impl City {
    fn change_name(&mut self, name: &str) {
        let former = mem::replace(&mut self.name, name.to_string());
        println!("{former} is now called {new}.", new = self.name);
    }
}

#[derive(Debug)]
struct Ring {
    owner: String,
    former_owners: Vec<String>,
}

impl Ring {
    fn switch_owner_to(&mut self, name: &str) {
        if let Some(position) = self.former_owners.iter().position(|n| n == name) {
            mem::swap(&mut self.owner, &mut self.former_owners[position])
        } else {
            println!("Nobody named {name} found in former_owners, sorry!");
        }
    }
}
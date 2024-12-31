fn main() {
    macro_rules! print_what_it_is {
        () => {
            println!("A macro with no arguments")
        };
        ($e:expr) => {
            println!("A macro with an expression")
        };
        ($s:stmt) => {
            println!("A macro with a statement")
        };
        ($e:expr, $s:stmt) => {
            println!("An expression followed by a statement")
        };
        ($e:stmt, $s:stmt) => {
            println!("Two back-to-back statements")
        };
    }
    
    macro_rules! noop_macro {
        () => {};
    }

    println!("Hello, world!");
    noop_macro!();
    print_what_it_is!();
    print_what_it_is!({});
    print_what_it_is!(;);
    print_what_it_is!({}, ;);
    print_what_it_is!(;, ;);

    macro_rules! format {
        ($($arg:tt)*) => {{
            let res = std::fmt::format(
                std::format_args!($($arg)*));
            res
        }}
    }

    macro_rules! special_println {
        ($($arg:tt)*) => {
        println!("Printed specially: {}", format!($($arg)*))
        };
    }

    special_println!("Hello, world2 {}!", 5);

    macro_rules! var_print {
        ($($v:ident),*) => {
            println!(
                concat!($(stringify!($v),"={:?} "),*), $($v),*
            )
        };
    }

    let counter = 7;
    let gauge = core::f64::consts::PI;
    let name = "Peter";
    var_print!(counter, gauge, name);

    let Poodle1 = Poodle::new("Poodle1", 5);
    println!("{} is a {} year old {}.", Poodle1.name, Poodle1.age, Poodle1.breed);

    let lab2 = Labrador2::new("lab2", 5);
    println!("{} is a {} year old {}.", lab2.name, lab2.age, lab2.breed);

    let container = Container {
        name: "Henry".into(),
    };

    get_name_from_first(&container);
    get_name_from_second(&container);

}

struct Container {
    name: String,
}
trait First {
    fn name(&self) {}
}

trait Second {
    fn name(&self, _: bool) {}
}

impl First for Container {
    fn name(&self) {}
}

impl Second for Container {
    fn name(&self, _: bool) {}
}

fn get_name_from_first<T: First>(t: &T) {
    t.name()
}

fn get_name_from_second<T: Second>(t: &T) {
    t.name(true)
}

trait Dog {
    fn name(&self) -> &String;
    fn age(&self) -> i32;
    fn breed(&self) -> &String;
}

macro_rules! dog_struct2 {
    ($breed:ident) => {
    struct $breed {
        name: String,
        age: i32,
        breed: String,
    }

    impl $breed {
        fn new(name: &str, age: i32) -> Self {
            Self {
                name: name.into(),
                age,
                breed: stringify!($breed).into(),
            }
        }
    }

    impl Dog for $breed {
        fn name(&self) -> &String {
            &self.name
        }
        fn age(&self) -> i32 {
            self.age
        }
        fn breed(&self) -> &String {
            &self.breed
        }
    }
};
}    

macro_rules! dog_struct {
    ($breed:ident) => {
        struct $breed {
            name: String,
            age: i32,
            breed: String,
        }

        impl $breed {
            fn new(name: &str, age: i32) -> Self {
                Self {
                    name: name.into(),
                    age,
                    breed: stringify!($breed).into(),
                }
            }
        }
    };
}

dog_struct!(Labrador); 
dog_struct!(Golden);
dog_struct!(Poodle); 

dog_struct2!(Labrador2);


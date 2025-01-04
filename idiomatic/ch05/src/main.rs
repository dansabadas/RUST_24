use std::thread;

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

    let builder = thread::Builder::new().stack_size(4 * 1024 * 1024); // 4 MB stack
    let handler = builder.spawn(|| {
        // Thread logic
    }).unwrap();

    handler.join().unwrap();

    // let mut bicycle_builder = BicycleBuilder::new();
    // bicycle_builder.with_make("Huffy");
    // bicycle_builder.with_model("Radio");
    // bicycle_builder.with_size(46);
    // bicycle_builder.with_color("red");
    // let bicycle = bicycle_builder.build();//was moved cannot be used anymore
    // println!("My new bike: {:#?}", bicycle);

    let bicycle2 = Bicycle::builder()
        .with_make("Trek")
        .with_model("Madone")
        .with_size(52)
        .with_color("purple")
        .build();
    println!("{:?}", bicycle2);
}

macro_rules! with_str {
    ($name:ident, $func:ident) => {
        pub fn $func(self, $name: &str) -> Self {
            Self {
                bicycle: Bicycle {
                    $name: $name.into(),
                    ..self.bicycle
                },
            }
        }
    };
}
macro_rules! with {
    ($name:ident, $func:ident, $type:ty) => {
        pub fn $func(self, $name: $type) -> Self {
            Self {
                bicycle: Bicycle {
                    $name,
                    ..self.bicycle
                },
            }
        }
    };     
}
macro_rules! accessor {
    ($name:ident, &$ret:ty) => {
        pub fn $name(&self) -> &$ret {
            &self.$name
        }
    };
    ($name:ident, $ret:ty) => {
        pub fn $name(&self) -> $ret {
            self.$name
        }
    };
}

#[derive(Debug)]
pub struct Bicycle {
    make: String,
    model: String,
    size: i32,
    color: String,
}

impl Bicycle {
    accessor!(make, &String);
    accessor!(model, &String);
    accessor!(size, i32);
    accessor!(color, &String);
    // fn make(&self) -> &String {
    //     &self.make
    // }
    // fn model(&self) -> &String {
    //     &self.model
    // }
    // fn size(&self) -> i32 {
    //     self.size
    // }
    // fn color(&self) -> &String {
    //     &self.color
    // }
}

pub trait Builder<T> {
    fn new() -> Self;
    fn build(self) -> T;
}

trait Buildable<Target, B: Builder<Target>> {
    fn builder() -> B;
}

pub struct BicycleBuilder {
    bicycle: Bicycle,
}

impl BicycleBuilder {
    // fn with_make(&mut self, make: &str) {
    //     self.bicycle.make = make.into()
    // }
    // fn with_model(&mut self, model: &str) {
    //     self.bicycle.model = model.into()
    // }
    // fn with_size(&mut self, size: i32) {
    //     self.bicycle.size = size
    // }
    // fn with_color(&mut self, color: &str) {
    //     self.bicycle.color = color.into()
    // }
    with_str!(make, with_make);
    with_str!(model, with_model);
    with!(size, with_size, i32);
    with_str!(color, with_color);
}

impl Builder<Bicycle> for BicycleBuilder {
    fn new() -> Self {
        Self {
            bicycle: Bicycle {
                make: String::new(),
                model: String::new(),
                size: 0,
                color: String::new(),
            },
        }
    }

    fn build(self) -> Bicycle {
        self.bicycle
    }
}    

impl Buildable<Bicycle, BicycleBuilder> for Bicycle {
    fn builder() -> BicycleBuilder {
        BicycleBuilder::new()
    }
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


use std::cell::RefCell;

use std::thread;
//use std::rc::Weak;// 
use std::sync::{Arc, Weak};

use std::fs::File;
use std::io::{BufRead, BufReader, Error, Seek, Write};

fn main() -> Result<(), Error> {
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

    let my_callback = || println!("I have been called back");
    callback_fn(my_callback);
//////////////////////////////
    // Create an Arc to manage shared data
    let data = Arc::new(42);

    // Downgrade the Arc to a Weak reference
    let weak_data: Weak<i32> = Arc::downgrade(&data);

    // Upgrade the Weak reference back to an Arc
    if let Some(upgraded_data) = weak_data.upgrade() {
        println!("Upgraded value: {}", upgraded_data);
    } else {
        println!("The value has been dropped.");
    }

    // Drop the original Arc
    drop(data);

    // Try upgrading again after the Arc is dropped
    if let Some(upgraded_data) = weak_data.upgrade() {
        println!("Upgraded value: {}", upgraded_data);
    } else {
        println!("The value has been dropped."); // This will execute
    }
//////////////////////////////
    // Create a parent node
    let parent = Arc::new(Node {
        value: 10,
        parent: RefCell::new(Weak::new()), // Initially no parent
    });

    // Create a child node with a reference to the parent
    let child = Arc::new(Node {
        value: 20,
        parent: RefCell::new(Arc::downgrade(&parent)), // Downgrade to a Weak reference
    });

    // Upgrade the weak reference from the child to access the parent
    if let Some(parent_ref) = child.parent.borrow().upgrade() {
        println!("Child's parent value: {}", parent_ref.value);
    } else {
        println!("Parent has been dropped.");
    }

    // Drop the parent node
    drop(parent);

    // Try upgrading again
    if let Some(parent_ref) = child.parent.borrow().upgrade() {
        println!("Child's parent value: {}", parent_ref.value);
    } else {
        println!("Parent has been dropped."); // This will execute
    };

    let mut subject = Subject::new("some subject state");
    let observer1 = MyObserver::new("observer1");
    let observer2 = MyObserver::new("observer2");
    subject.attach(observer1.clone());
    subject.attach(observer2.clone());
    // ... do something here ...
    subject.update();
    //////////////////////////////
    let file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("file.txt")?;
    let commands: Vec<Box<dyn Command>> = vec![
        ReadFile::new(file.try_clone()?),
        WriteFile::new(
            "file content\n".into(), file.try_clone()?
        ),
        ReadFile::new(file.try_clone()?),
    ];
    for command in commands {
        command.execute()?;
    }

    Ok(())
}

trait Command {
    fn execute(&self) -> Result<(), Error>;
}

struct ReadFile {
    receiver: File,
}

impl ReadFile {
    fn new(receiver: File) -> Box<Self> {
        Box::new(Self { receiver })
    }
}

impl Command for ReadFile {
    fn execute(&self) -> Result<(), Error> {
        println!("Reading from start of file");
        let mut reader = BufReader::new(&self.receiver);
        reader.seek(std::io::SeekFrom::Start(0))?;
        for (count, line) in reader.lines().enumerate() {
            println!("{:2}: {}", count + 1, line?);
        }

        Ok(())
    }
}

struct WriteFile {
    content: String,
    receiver: File,
}

impl WriteFile {
    fn new(content: String, receiver: File) -> Box<Self> {
        Box::new(Self { content, receiver })
    }
}

impl Command for WriteFile {
    fn execute(&self) -> Result<(), Error> {
        println!("Writing new content to file");
        let mut writer = self.receiver.try_clone()?;
        writer.write_all(self.content.as_bytes())?;
        writer.flush()?;

        Ok(())
    }
}    

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // Weak reference to avoid circular dependency
}

pub trait Observer {
    type Subject;
    fn observe(&self, subject: &Self::Subject);
}

pub trait Observable {
    type Observer;
    fn update(&self);
    fn attach(&mut self, observer: Self::Observer);
    fn detach(&mut self, observer: Self::Observer);
}

pub struct Subject {
    observers: Vec<Weak<dyn Observer<Subject = Self>>>,
    state: String,
}

impl Subject {
    pub fn new(state: &str) -> Self {
        Self {
            observers: vec![],
            state: state.into(),
        }
    }
    
    pub fn state(&self) -> &str {
        self.state.as_ref()
    }
}

impl Observable for Subject {
    type Observer = Arc<dyn Observer<Subject = Self>>;
    fn update(&self) {
        self.observers
            .iter()
            .flat_map(|o| o.upgrade())
            .for_each(|o| o.observe(self));
    }
    fn attach(&mut self, observer: Self::Observer) {
        self.observers.push(Arc::downgrade(&observer));
    }
    fn detach(&mut self, observer: Self::Observer) {
        self.observers
            .retain(|f| {
                !f.ptr_eq(&Arc::downgrade(&observer))
            });
    }
}

struct MyObserver {
    name: String,
}

impl MyObserver {
    fn new(name: &str) -> Arc<Self> {
        Arc::new(Self { name: name.into() })
    }
}

impl Observer for MyObserver {
    type Subject = Subject;
    fn observe(&self, subject: &Self::Subject) {
        println!(
            "observed subject with state={:?} in {}",
            subject.state(),
            self.name
        );
    }
}

fn callback_fn<F>(f: F) where F: Fn() -> (),
{
    f();
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


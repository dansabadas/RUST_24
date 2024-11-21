macro_rules! give_six {
    () => {
        6
    };
}

macro_rules! pure_nonsense {
    (Hi Calvin.) => {
        GRITTINGS. MA NAM IS KAHLFIN. HEERYOR LUNBOKS. HOFFA GUT TAY ASKOOL.
    };
}

macro_rules! six_or_print {
    (6) => {
        6
    };
    () => {
        println!("You didn't give me 6.");
    };
}

macro_rules! might_print {
    (THis is strange input 하하はは哈哈 but it still works) => {
        println!("You guessed the secret message!")
    };
    () => {
        println!("You didn't guess it");
    };
}

macro_rules! might_print2 {
    ($input:expr) => {
        println!("You gave me: {:?}", $input);
    }
}

macro_rules! wants_expression {
    ($input:stmt) => {
        println!("You matched the macro input!");
    };
}

macro_rules! check {
    ($input1:ident, $input2:expr) => {
        println!(
            "Is {:?} equal to {:?}? {:?}",
            $input1,
            $input2,
            $input1 == $input2
        );
    };
}

macro_rules! print_anything {
    ($input:tt) => {
        let output = stringify!($input);
        println!("{}", output);
    };
}

macro_rules! print_anything2 {
    ($($input1:tt),*) => {
        let output = stringify!($($input1),*);
        println!("{}", output);
    };
}

macro_rules! make_a_function {
    ($name:ident, $($input:tt),+) => {
        fn $name() {
            let output = stringify!($($input),+);
            println!("{}", output);
        }
    };
}

fn main() {
    let six = give_six!();
    println!("{}", six);

    //let x = pure_nonsense!(Hi Calvin.);
    let my_number = six_or_print!(6);
    println!("{}", my_number);
    six_or_print!();

    might_print!(THis is strange input 하하はは哈哈 but it still works);
    might_print!();

    might_print2!(67);
    might_print2!(());
    might_print2!(6);
    might_print2!(vec![8, 9, 7, 10]);

    wants_expression!(let x = 9);

    let x = 6;
    let my_vec = vec![7, 8, 9];
    check!(x, 6);
    check!(my_vec, vec![7, 8, 9]);
    check!(x, 10);
    print_anything!(ththdoetd);
    print_anything!(87575oehq75onth);

    print_anything2!(ththdoetd,rcofe);
    print_anything2!();
    print_anything2!(87575oehq75onth, ntohe, 987987o, 097);

    make_a_function!(print_it, 5, 5, 6, I);
    print_it();
    make_a_function!(say_its_nice, this, is, really, nice);
    say_its_nice();
}

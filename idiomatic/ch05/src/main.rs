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
}



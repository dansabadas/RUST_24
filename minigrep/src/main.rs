use std::env;
use std::fs;
use std::process;
use std::error::Error;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // let query = &args[1];
    // let file_path = &args[2];

    // println!("Searching for {query}");
    // println!("In file {file_path}");

    // let contents = fs::read_to_string(file_path)
    //     .expect("Should have been able to read the file");

    // println!("With text:\n{contents}");

    // let (query, file_path) = parse_config0(&args);

    // let original = String::from("Hello, world!");  
    // let reference = &original;  
    
    // // Clone the &String into a fully owned String  
    // let owned_string = reference.clone();  
    
    // println!("{}", owned_string); // Output: Hello, world!  

    // let config = parse_config(&args);
    // println!("{:?}", config.file_path); // Output: Hello, world!  

    //let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing: {err}");
        process::exit(1);
    });

    //print!("{}", run(config));
    if let Err(e) = minigrep::run(config) {
        eprint!("App error {e}");
        process::exit(1);
    }
}

// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.file_path)?;
//         //.expect("Should have been able to read the file");

//     println!("With text:\n{contents}");
//     Ok(())
// }

// struct Config {
//     query: String,
//     file_path: String
// }

// impl Config {
//     fn build(args: &[String]) -> Result<Config, &'static str>{
//         if args.len() < 3 {
//             //print!("args.len()={}", args.len());

//             return Err("not enough args");
//         }

//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Ok(Config{query, file_path})
//     }

//     fn new(args: &[String]) -> Config {
//         if args.len() != 2 {
//             panic!("not enough args");
//         }
//         Config { 
//             query: args[1].to_owned(), 
//             file_path: args[2].to_owned() 
//         }
//     }
// }

fn parse_config(args: &[String]) -> Config {
    let query = args[1].to_owned();
    let file_path = args[2].to_owned();

    Config { query, file_path, ignore_case: true }
}

fn parse_config0(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
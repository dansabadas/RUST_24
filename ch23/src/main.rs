use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{Clear, ClearType},
};
use std::{fs::read_to_string, io::stdout};

struct App {
    file_content: String,
    user_input: String,
}
impl App {
    fn new(file_name: &str) -> Result<Self, std::io::Error> {
        let file_content = read_to_string(file_name)?;
        Ok(Self {
            file_content,
            user_input: String::new(),
        })
    }
}

fn main() -> Result<(), std::io::Error> {
    // loop {
    //     let event = read().unwrap();
    //     println!("{:?}", event);
    // }
    // let file_content = read_to_string("typing.txt").unwrap();
    // let mut user_input = String::new();
    // loop {
    //     println!("{file_content}");
    //     println!("{user_input}_");
    //     if let Event::Key(key_event) = read().unwrap() {
    //         if key_event.kind == KeyEventKind::Press {
    //             match key_event.code {
    //                 KeyCode::Backspace => {
    //                     user_input.pop();
    //                 }
    //                 KeyCode::Esc => break,
    //                 KeyCode::Char(c) => {
    //                     user_input.push(c);
    //                 }
    //                 _ => {}
    //             }
    //         }
    //     }
    // }
    let mut app = App::new("typing.txt")?;
    loop {
        println!("{}", app.file_content);
        for (letter1, letter2) in
        app.user_input.chars().zip(app.file_content.chars()) {
            if letter1 == letter2 {
                print!("{letter2}");
            } else {
                print!("*");
            }
        }
        println!("_");
        if let Event::Key(key_event) = read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Backspace => {
                        app.user_input.pop();
                    }
                    KeyCode::Esc => break,
                    KeyCode::Char(c) => {
                        app.user_input.push(c);
                    }
                    KeyCode::Enter => {
                        let total_chars = app.file_content.chars().count();
                        let total_right = app
                            .user_input
                            .chars()
                            .zip(app.file_content.chars())
                            .filter(|(a, b)| a == b)
                            .count();
                        println!("You got {total_right} out of {total_chars}!");
                        return Ok(());
                    }
                    _ => {}
                }
            }
            execute!(stdout(), Clear(ClearType::All))?;
        }
    }
    Ok(())
}

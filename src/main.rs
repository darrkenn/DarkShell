mod input_handling;
use std::{env,io};
use std::env::home_dir;
use std::io::{stdout, Write};
use std::str::Chars;
use ansi_term::Color::{Blue, Green};
use whoami::username;
use crate::input_handling::handle_input;
use console::{Key, Term};
fn main() {
    let home = home_dir().unwrap();
    env::set_current_dir(&home).expect("Uhoh cant set dir to home.");
    println!("Welcome to DarkShell!");
    let username = username().unwrap().to_string();
    let mut past_commands: Vec<String> = Vec::new();

    loop {
        handle_current_dir(username.clone());
        stdout().flush().unwrap();
        let input = handle_user_input(&mut past_commands);
        past_commands.push(input.clone());
        let parsed: Vec<&str> = input.trim().split(' ').collect();
        handle_input(parsed, home.clone()).expect("Cant process input");
    }
}
fn handle_current_dir(username: String) {
    match env::current_dir() {
        Ok(path) => {
            if let Some(dir_name) = path.file_name() {
                print!("{}", format!("{}@{}{}",Green.paint(username), Green.paint( dir_name.to_string_lossy()),Blue.paint(">> ")));
            } else {
                //Handles root directory better somehow idk.
                print!("{}", format!("{}{}", Green.paint("/"),Blue.paint(">> ")));
            }
        }
        Err(err) => {println!("{}", err)}
    }
}
fn handle_user_input(past_commands: &mut Vec<String>) -> String {
    let term = Term::stdout();
    let mut chars: Vec<char> = Vec::new();
    let mut iter: u8 = 0;
    loop {
        let key = term.read_key().unwrap();
        match key {
            Key::Char(c) => {
                chars.push(c);
                print!("{c}");
                std::io::Write::flush(&mut stdout()).unwrap();
            }
            Key::ArrowUp =>{
                iter+=1;
                if past_commands.len() > iter as usize -1  {
                    let command = &past_commands[iter as usize - 1];

                    print!("{command}");
                    std::io::Write::flush(&mut stdout()).unwrap();
                } else {
                    print!("test");
                    std::io::Write::flush(&mut stdout()).unwrap();
                }
            }
            Key::ArrowDown => {
                if iter != 0 {
                    iter -= 1;
                    if past_commands.len() < iter as usize -1  {
                        let command = &past_commands[iter as usize - 1];

                        print!("{command}");
                        std::io::Write::flush(&mut stdout()).unwrap();
                    } else {
                        print!("test");
                        std::io::Write::flush(&mut stdout()).unwrap();
                    }
                } else {
                    if past_commands.len() != 0 {
                        let command = &past_commands[0];
                        print!("{command}");
                        std::io::Write::flush(&mut stdout()).unwrap();
                    } else {
                        print!("is nothing");
                        std::io::Write::flush(&mut stdout()).unwrap();
                    }
                }
            }
            Key::Backspace => {
                if !chars.is_empty() {
                    chars.pop();
                    print!("\x08 \x08");
                    std::io::Write::flush(&mut stdout()).unwrap();
                }
            }
            Key::Enter => {
                let input = chars.iter().collect::<String>();
                println!();
                return input
            }
            _ => {

            }
    }
}
    // io::stdin().read_line(&mut input).unwrap();
}




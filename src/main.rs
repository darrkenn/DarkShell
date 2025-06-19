mod input_handling;
use std::{env};
use std::env::home_dir;
use std::io::{stdout, Write};
use ansi_term::Color::{Blue, Green};
use whoami::username;
use crate::input_handling::{handle_command_input, handle_input};
fn main() {
    let home = home_dir().unwrap();
    env::set_current_dir(&home).expect("Uhoh cant set dir to home.");
    println!("Welcome to DarkShell!");
    let username = username().unwrap().to_string();
    let mut past_commands: Vec<String> = Vec::new();
    loop {
        handle_current_dir(username.clone());
        stdout().flush().unwrap();
        let input = handle_command_input(&mut past_commands);
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





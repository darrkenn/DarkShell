mod input_handling;

use std::{env, io};
use std::env::home_dir;
use std::io::{stdout, Write};
use ansi_term::Color::Blue;
use crate::input_handling::handle_input;


fn main() {
    let home = home_dir().unwrap();
    env::set_current_dir(&home).expect("Uhoh cant set dir to home.");
    println!("Welcome to DarkShell!");
    loop {
        let mut input = String::new();
        print!("{}", Blue.paint(">> "));
        stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let parsed: Vec<&str> = input.trim().split(' ').collect();

        handle_input(parsed, home.clone()).expect("Cant process input");
    }
}
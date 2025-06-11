mod input_handling;

use std::io;
use crate::input_handling::handle_input;

fn main() {
    println!("Welcome to DarkShell!");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parsed: Vec<&str> = input.trim().split(' ').collect();

        handle_input(parsed);
    }
}
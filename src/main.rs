mod input_handling;

use std::{env, io};
use std::io::{stdout, Write};
use std::path::Path;
use crate::input_handling::handle_input;

fn main() {
    let root = Path::new("/");
    env::set_current_dir(&root).is_ok();
    println!("Welcome to DarkShell!");
    loop {
        let mut input = String::new();
        print!(">> ");
        stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let parsed: Vec<&str> = input.trim().split(' ').collect();

        handle_input(parsed);
    }
}
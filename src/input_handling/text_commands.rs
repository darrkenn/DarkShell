use std::{io};
use std::io::stdout;
use ansi_term::Color::{Green};
use ferris_says::say;
pub fn handle_say(args: &[&str], length: usize) -> Result<(), Box<dyn std::error::Error>> {
    match args {
        [arg] if length == 2 => {
            let word = arg.to_string();
            let width = word.len();
            let mut writer = io::BufWriter::new(stdout());
            say(&*Green.paint(word), width, &mut writer).unwrap();
        }
        args if length > 2 => {
            let string: Vec<&str> = args.to_vec();
            let word = string.join(" ");
            let width = word.len();
            let mut writer = io::BufWriter::new(stdout());
            say(&*Green.paint(word), width, &mut writer).unwrap();
        }
        _ => {println!("Invalid choice");}
    }
    Ok(())
}
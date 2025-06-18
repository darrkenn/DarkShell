use std::{fs, io};
use std::io::stdout;
use std::path::Path;
use ansi_term::Color::{Green, Red};
use ferris_says::say;

pub fn handle_echo(args: &[&str], length: usize) -> Result<(), Box<dyn std::error::Error>> {
    match args {
        [arg] if length == 2 => {
            println!("{}", arg);
        }
        args if length > 2 => {
            let string: Vec<&str> = args.to_vec();
            println!("{}", string.join(" "));
        }
        _ => {
            println!("{}", Red.paint("Invalid cat!"));
        }
    }
    Ok(())
}

pub fn handle_cat(arg: &str, length: usize) -> Result<(), Box<dyn std::error::Error>> {
    match arg {
        arg if Path::new(arg).exists() => {
            let content = fs::read_to_string(arg)?;
            println!("{}", content);
        }
        _ => {
            println!("{}", Red.paint("File not found!"));
        }
    }
    Ok(())
}

pub fn handle_grep(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(&args[1]).exists() {
        let content = fs::read_to_string(&args[1])?;
        let lines: Vec<&str> = content.lines().collect();
        for line in lines.iter() {
            if line.contains(args[0]) {
                let highlighted_line = line.replace(args[0], &format!("{}", Green.underline().paint(args[0])));
                println!("{}", highlighted_line);
            }

        }
    } else {
        println!("Invalid input");
    }
    Ok(())
}

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
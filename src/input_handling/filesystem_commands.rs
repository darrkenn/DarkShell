use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use ansi_term::Color::Red;
use crate::input_handling::directory_commands::handle_ls;

pub fn handle_cp(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let file = args[0];
    let destination = args[1];
    let dest_path = Path::new(destination);
    if fs::exists(file)? {
        let final_dest = if dest_path.is_dir() {
            let file_path = Path::new(file);
            let file_name = file_path.file_name().ok_or("Invalid path!")?;
            dest_path.join(file_name)
        } else {
            dest_path.to_path_buf()
        };
        fs::copy(file, &final_dest)?;
    } else {
        println!("{}", Red.paint("File not found"));
    }
    Ok(())
}
pub fn handle_mv(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let file = args[0];
    let destination = args[1];
    let dest_path = Path::new(destination);
    if fs::exists(file)? {
        fs::rename(file, &dest_path)?;
    } else {
        println!("{}", Red.paint("File not found"));
    }
    Ok(())
}
pub fn handle_rm(file: &str) -> Result<(), Box<dyn std::error::Error>> {
    if fs::exists(file)? {
        fs::remove_file(file)?;
    } else {
        println!("{} does not exist", file);
    }
    Ok(())
}

pub fn handle_find(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    if fs::exists(args[1])? {
        let entries = handle_ls(args[1].parse().unwrap())?;
        let word = args[0];
        for entry in entries.iter() {
            if word.ends_with("/") && entry != "/" && entry == word {
                println!("Found {}", entry);
            } else if !word.ends_with("/") && entry == word  {
                println!("{}", entry);
            }
        }
    } else {
        println!("{}", Red.paint("Not a directory."));
    }
    Ok(())
}

pub fn handle_head(file: &str) -> Result<(), Box<dyn std::error::Error>> {
    if fs::exists(file)? {
        let content = fs::read_to_string(file)?;
        if let Some(head) = content.lines().next() {
            println!("{}", head);
        }
    } else {
        println!("{} does not exist", file);
    }
    Ok(())
}
pub fn handle_tail(file: &str) -> Result<(), Box<dyn std::error::Error>> {
    if fs::exists(file)? {
        let content = fs::read_to_string(file)?;
        if let Some(head) = content.lines().last() {
            println!("{}", head);
        }
    } else {
        println!("{} does not exist", file);
    }
    Ok(())
}

pub fn handle_fs(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let file = args[0];
    let choice = args[1];
    let content = fs::read_to_string(file)?;
    if fs::exists(file)? {
        match choice {
            "-words" => {
                let word_count = content.split_whitespace().count();
                println!("There is {} words in {}", word_count, file);
            },
            "-lines" => {
                let line_count = content.lines().count();
                println!("There is {} lines in {}", line_count, file);
            },
            "-chars" => {
                let chars_count = content.chars().count();
                println!("There is {} chars in {}", chars_count, file);
            },
            _ => { println!("Invalid choice") }
        }

    } else {
        println!("{} does not exist", file);
    }
    Ok(())
}

pub fn handle_touch(file: &str) -> Result<(), Box<dyn std::error::Error>> {
    File::create(file)?;
    Ok(())
}
pub fn check_if_path(path: PathBuf) -> bool {
    if path.exists() && path.is_dir() {
        true
    } else {
        false
    }
}
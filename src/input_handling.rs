use std::{env, fs, io};
use std::path::{Path, PathBuf};
use ansi_term;
use ansi_term::Color::{Green, Red};

pub fn handle_input(input: Vec<&str>, home: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let current_path = env::current_dir()?;
    let length = input.len();
    match input.as_slice() {
        ["cat"] => handle_cat(&[], length),
        ["cat", args @ ..] => handle_cat(args, length),
        ["clear"] => handle_clear(),
        ["ls"] => handle_current_ls(current_path),
        ["ls", arg]  if !arg.is_empty() && length == 2 => handle_different_ls(arg.parse().unwrap()),
        ["cd"] => handle_cd(home),
        ["cd", arg] if !arg.is_empty() && length == 2 => handle_cd(arg.parse().unwrap()),
        ["grep", args @ ..] if !args.is_empty() && length == 3 => handle_grep(args),
        _ => {
            println!("{}", Red.paint("Invalid input!"));
            Ok(())
        }
    }
}

fn handle_cat(args: &[&str], length: usize) -> Result<(), Box<dyn std::error::Error>> {
    match args {
        [] => {
            println!("Provide text");
        }
        [arg] if length == 2 => {
            if Path::new(arg).exists() {
                let content = fs::read_to_string(arg)?;
                println!("  {}", content);
            } else {
                println!("{}", arg);
            }
        }
        args if length > 2 => {
            let string: Vec<&str> = args.to_vec();
            println!("{}", string.join(" "));
        }
        _ => {
            println!("{}", Red.paint("Invalid input"));
        }
    }
    Ok(())
}

fn handle_grep(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
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


fn handle_cd(destination: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    env::set_current_dir(destination)?;
    Ok(())
}


fn handle_different_ls(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let entries = handle_ls(path)?;
    for entry in entries.iter() {
        println!("  {}", entry);
    }
    Ok(())
}

fn handle_current_ls(current_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let entries = handle_ls(current_path)?;
    for entry in entries.iter() {
        println!("  {}", entry);
    }
    Ok(())
}

fn handle_dir(current_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", current_path.display());
    Ok(())
}


fn handle_clear() -> Result<(), Box<dyn std::error::Error>> {
    clear_screen::clear();
    Ok(())
}





fn handle_ls(path: PathBuf) -> io::Result<Vec<String>> {
    let metadata = fs::metadata(&path)?;

    if metadata.is_file() {
        if let Some(file_name) = path.file_name() {
            if let Some(name) = file_name.to_str() {
                return Ok(vec![name.to_owned()]);
            }
        }
        return Ok(vec![]);
    }

    if !metadata.is_dir() {
        return Ok(vec![]);
    }

    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str() {
            if entry.metadata()?.is_dir() {
                files.push(format!("{}/", name));
            } else {
                files.push(name.to_string());
            }
        }
    }
    Ok(files)
}
use std::path::{Path, PathBuf};
use std::{env, fs, io, process};
use std::fmt::format;
use std::fs::File;
use std::io::stdout;
use std::process::{Command, Stdio};
use std::ptr::replace;
use ansi_term::Color::{Green, Red};
use sysinfo::{System};
use ferris_says::say;

pub fn handle_exit() -> Result<(), Box<dyn std::error::Error>> {
    process::exit(1);
}

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

pub fn handle_cat(args: &[&str], length: usize) -> Result<(), Box<dyn std::error::Error>> {
    match args {
        [arg] if length == 2 => {
            if Path::new(arg).exists() {
                let content = fs::read_to_string(arg)?;
                println!("{}", content);
            } else {
                println!("{}", arg);
            }
        }
        _ => {
            println!("{}", Red.paint("Invalid input"));
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
pub fn handle_cd(destination: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if destination.exists() {
        env::set_current_dir(destination)?;
    } else {
        println!("{} isnt a directory", destination.display());
    }
    Ok(())
}
pub fn handle_different_ls(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let entries = handle_ls(path)?;
    for entry in entries.iter() {
        println!("  {}", entry);
    }
    Ok(())
}

pub fn handle_current_ls(current_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let entries = handle_ls(current_path)?;
    for entry in entries.iter() {
        println!("  {}", entry);
    }
    Ok(())
}

pub fn handle_pwd(current_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", current_path.display());
    Ok(())
}

pub fn handle_clear() -> Result<(), Box<dyn std::error::Error>> {
    clear_screen::clear();
    Ok(())
}

pub fn handle_ls(path: PathBuf) -> io::Result<Vec<String>> {
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

pub fn handle_mkdir(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir(path)?;
    Ok(())
}
pub fn handle_rmdir(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    fs::remove_dir(path)?;
    Ok(())
}
pub fn handle_rmdir_all(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    fs::remove_dir_all(path)?;
    Ok(())
}
pub fn handle_touch(file: &str) -> Result<(), Box<dyn std::error::Error>> {
    File::create(file)?;
    Ok(())
}

pub fn handle_help() -> Result<(), Box<dyn std::error::Error>> {
    println!("couldnt  be bothered tbh");
    Ok(())
}

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

pub fn handle_single_ps(arg: &str) -> Result<(), Box<dyn std::error::Error>> {
    let choice = arg;
    let mut sys = System::new_all();
    sys.refresh_all();
    match choice {
        "-n" => {
            println!("There are {} active processes", sys.processes().len());
        }
        "-l" => {
            for (pid, process) in sys.processes() {
                println!("[{pid}] {:?} ", process.name(),);
            }
        }
        _ => {println!("Invalid choice")}
    }
    Ok(())
}
pub fn handle_getorkill_ps(arg: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let choice = arg[0];
    let getorkill = arg[1];
    let mut sys = System::new_all();
    sys.refresh_all();
    match choice {
        "-g" => {
            for (pid, process) in sys.processes() {
                if process.name().to_str() == Option::from(getorkill) {
                    println!("[{pid}] {:?} ", process.name(),);
                } else {

                }
            }
        }
        "-k" => {
            let chosen_id = match getorkill.parse::<u32>() {
                Ok(id) => id,
                Err(_) => {
                    println!("Error: '{}' not a valid PID", getorkill);
                    return Ok(());
                }
            };
            let mut found = false;
            for (pid, process) in sys.processes() {
                if pid.as_u32() == chosen_id {
                    process.kill();
                    println!("Successfully killed {:?} (PID: {})", process.name(), chosen_id);
                    found = true;
                    break;
                }
            }
            if !found {
                println!("No process found with PID: {}", chosen_id);
            }
        }
        _ => {println!("Invalid choice")}
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
pub fn handle_external_command(user_input: String) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(user_input.clone())
        .env("TERM", "xterm-256color")
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&*output.stdout).into_owned();
        println!("{}", stdout);
    } else if !output.status.success() {
        {
            let stderr = String::from_utf8_lossy(&*output.stderr).into_owned();
            // let error = format!("{}: {} not found", "dksh", user_input);
            let mut parsed_err: Vec<&str> = stderr.trim().split(' ').collect();
            let mut test:u64 = 0;
            if parsed_err[0].contains("sh:") {
                parsed_err[0] = "dksh:";
                parsed_err.remove(1);
                parsed_err.remove(2);
                parsed_err[1] = user_input.as_str();
                parsed_err[2] = "not found:(";
            };
            let error = parsed_err.join(" ");
            println!("{}",error);
        }
    }
}
pub fn check_if_path(path: PathBuf) -> bool {
    if path.exists() && path.is_dir() {
        true
    } else {
        false
    }
}
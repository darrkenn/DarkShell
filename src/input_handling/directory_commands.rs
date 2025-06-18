use std::{env, fs, io};
use std::path::PathBuf;

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

pub fn handle_cd(destination: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if destination.exists() {
        env::set_current_dir(destination)?;
    } else {
        println!("{} isnt a directory", destination.display());
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
pub fn handle_different_ls(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let entries = handle_ls(path)?;
    for entry in entries.iter() {
        println!("  {}", entry);
    }
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

pub fn handle_pwd(current_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", current_path.display());
    Ok(())
}
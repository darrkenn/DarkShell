use std::{env, fs, io};
use std::path::PathBuf;
pub fn handle_input(input: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let default_path = env::current_dir()?;
    let length = input.len();
    match input.as_slice() {
        /////////////////////
        ////    Cat      ////
        /////////////////////
        ["cat", args @ ..] if !args.is_empty() && length > 2 => {
            let string: Vec<&str> = <[&str]>::into_vec(Box::from(args));
            println!("{}", string.join(" "));
        }
        ["cat", arg] if !arg.is_empty() && length == 2 => {
            println!("{}", arg);
        }
        ["cat"] => {
            println!("Provide text");
        }
        /////////////////////
        ////    Clear    ////
        /////////////////////
        ["clear"] => {
            clear_screen::clear();
        }
        /////////////////////
        ////    Dir      ////
        /////////////////////
        ["dir"] => {
            println!("{}", default_path.display());
            for file in fs::read_dir(default_path).unwrap() {
                println!("{}", file.unwrap().path().display());
            }
        }
        /////////////////////
        ////      Ls     ////
        /////////////////////
        ["ls"] => {
            let entries = handle_ls(default_path)?;
            for entry in entries.iter() {
                println!("{}", entry);
            }
        }
        ["ls", arg] if !arg.is_empty() && length == 2 => {
            let entries = handle_ls(arg.parse().unwrap())?;
            for entry in entries.iter() {
                println!("{}", entry);
            }
        }
        
        ["cd", arg] if !arg.is_empty() && length == 2 => {
            env::set_current_dir(arg)?;
        }



        _ => {
            println!("Invalid");
        }
    }
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
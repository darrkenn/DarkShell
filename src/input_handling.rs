mod misc_commands;
mod text_commands;
mod directory_commands;
mod process_commands;

use std::{env};
use std::path::{PathBuf};
use crate::input_handling::misc_commands::{handle_clear, handle_exit, handle_help, handle_external_command, };
use crate::input_handling::directory_commands::{handle_cd, handle_current_ls, handle_pwd,};
use crate::input_handling::process_commands::{handle_getorkill_ps, handle_single_ps};
use crate::input_handling::text_commands::{handle_say};

pub fn handle_input(input: Vec<&str>, home: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let current_path = env::current_dir()?;
    let length = input.len();
    let sliced_input = input.as_slice();
    if sliced_input[length - 1] == "*"  {

    }

    match sliced_input {
        ["say", args @ ..] if !args.is_empty() => handle_say(args, length),
        
        ["clear"] => handle_clear(),
        ["ls"] => handle_current_ls(current_path),
        ["cd"] => handle_cd(home),
        ["cd", arg] if !arg.is_empty() => handle_cd(arg.parse().unwrap()),
        ["pwd"] => handle_pwd(current_path),
        ["exit"] => handle_exit(),
        ["help"] => handle_help(),
        
        ["ps", arg] if !arg.is_empty() => handle_single_ps(arg),
        ["pid", args @ ..] if !args.is_empty() && length == 3 => handle_getorkill_ps(args),
        
        _ => {
            let command = input[0];
            let args = &input[1..];
            handle_external_command(command, args);
            Ok(())
        }
    }
}


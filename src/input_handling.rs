mod misc_commands;
mod text_commands;
mod filesystem_commands;
mod directory_commands;
mod process_commands;

use std::{env};
use std::path::{PathBuf};
use crate::input_handling::misc_commands::{handle_clear, handle_exit, handle_help, handle_external_command, };
use crate::input_handling::directory_commands::{handle_cd, handle_current_ls, handle_different_ls, handle_mkdir, handle_pwd, handle_rmdir, handle_rmdir_all};
use crate::input_handling::filesystem_commands::{check_if_path, handle_cp, handle_find, handle_fs, handle_head, handle_mv, handle_rm, handle_tail, handle_touch};
use crate::input_handling::process_commands::{handle_getorkill_ps, handle_single_ps};
use crate::input_handling::text_commands::{handle_cat, handle_echo, handle_grep, handle_say};

pub fn handle_input(input: Vec<&str>, home: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let current_path = env::current_dir()?;
    let length = input.len();
    match input.as_slice() {
        ///////////////////////
        //    Text output   //
        /////////////////////
        ["echo", args @ ..] => handle_echo(args, length),
        ["cat", arg] if length == 2 => handle_cat(arg, length),
        ["say", args @ ..] if !args.is_empty() => handle_say(args, length),
        
        ["clear"] => handle_clear(),
        ["ls"] => handle_current_ls(current_path),
        ["ls", arg]  if !arg.is_empty() =>  {
            let is_path = check_if_path(arg.parse().unwrap());
            if is_path == true {
                handle_different_ls(arg.parse().unwrap()).expect("Uh oh");
            } else if is_path == false {
                let command = input[0];
                let args = &input[1..];
                handle_external_command(command, args);
            }
            Ok(())
        },
        ["cd"] => handle_cd(home),
        ["cd", arg] if !arg.is_empty() => handle_cd(arg.parse().unwrap()),
        ["grep", args @ ..] if !args.is_empty() && length == 3 => handle_grep(args),
        ["pwd"] => handle_pwd(current_path),
        ["exit"] => handle_exit(),
        ["help"] => handle_help(),
        ///////////////////////
        //     Directory    //
        /////////////////////
        ["mkdir", arg] if !arg.is_empty() => handle_mkdir(arg.parse().unwrap()),
        ["rmdir", arg] if !arg.is_empty() => handle_rmdir(arg.parse().unwrap()),
        ["rmdirall", arg] if !arg.is_empty() => handle_rmdir_all(arg.parse().unwrap()),
        ///////////////////////
        //       Files      //
        /////////////////////
        ["touch", arg] if !arg.is_empty() => handle_touch(arg),
        ["rm", arg] if !arg.is_empty() => handle_rm(arg),
        ["cp", args @ ..] if !args.is_empty() && length == 3 => handle_cp(args),
        ["mv", args @ ..] if !args.is_empty() && length == 3 => handle_mv(args),
        ["find", args @ ..] if !args.is_empty() && length == 3 => handle_find(args),
        ["head", arg] if !arg.is_empty() => handle_head(arg),
        ["tail", arg] if !arg.is_empty() => handle_tail(arg),
        ["fs", args @ ..] if !args.is_empty() && length == 3 => handle_fs(args),
        ///////////////////////
        //       System     //
        /////////////////////
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


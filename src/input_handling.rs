mod command_handling;
use std::{env};
use std::path::{PathBuf};
use ansi_term;
use crate::input_handling::command_handling::{handle_cat, handle_cd, handle_clear, handle_cp, handle_current_ls, handle_different_ls, handle_echo, handle_exit, handle_grep, handle_help, handle_mkdir, handle_mv, handle_pwd, handle_rm, handle_rmdir, handle_rmdir_all, handle_find, handle_touch, handle_head, handle_tail, handle_fs, handle_single_ps, handle_getorkill_ps, handle_say, handle_external_command, check_if_path};


pub fn handle_input(input: Vec<&str>, home: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let current_path = env::current_dir()?;
    let length = input.len();
    match input.as_slice() {
        ///////////////////////
        //    Text output   //
        /////////////////////
        ["echo", args @ ..] => handle_echo(args, length),
        ["cat", args @ ..] => handle_cat(args, length),
        ["say", args @ ..] if !args.is_empty() => handle_say(args, length),
        
        
        ["clear"] => handle_clear(),
        ["ls"] => handle_current_ls(current_path),
        ["ls", arg]  if !arg.is_empty() =>  {
            let is_path = check_if_path(arg.parse().unwrap());
            if is_path == true {
                handle_different_ls(arg.parse().unwrap()).expect("Uh oh");
            } else if is_path == false {
                handle_external_command(input.join(" "));
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
            handle_external_command(input.join(" "));
            Ok(())
        }
    }
}


mod misc_commands;
mod text_commands;
mod directory_commands;
mod process_commands;
use std::{env};
use std::io::stdout;
use std::path::{PathBuf};
use console::{Key, Term};
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

pub fn handle_command_input(past_commands: &mut Vec<String>) -> String {
    let term = Term::stdout();
    let mut chars: Vec<char> = Vec::new();
    let mut iter: u8 = 0;
    let vec_length = past_commands.len();
    loop {
        let key = term.read_key().unwrap();
        match key {
            Key::Char(c) => {
                chars.push(c);
                print!("{c}");
                std::io::Write::flush(&mut stdout()).unwrap();
            }
            Key::ArrowUp =>{
                //Im sure theres a much better way to do this but its working and im not complaining
                if past_commands.is_empty() {continue}
                if vec_length == iter as usize{continue}
                if vec_length >= 1 {
                    for _c in chars.iter() {
                        print!("\x08 \x08");
                    }
                    iter+=1;
                    let command = &past_commands[vec_length-iter as usize];
                    chars.clear();
                    for  c in command.chars() {
                        chars.push(c);
                        print!("{c}");
                    }
                    std::io::Write::flush(&mut stdout()).unwrap();
                }
            }
            Key::ArrowDown => {
                //Yeah this is shady
                if past_commands.is_empty() {continue}
                if vec_length >= 1 {
                    if iter != 0 {
                        for _c in chars.iter() {
                            print!("\x08 \x08");
                        }
                        let command = &past_commands[vec_length-iter as usize];
                        iter-=1;
                        chars.clear();
                        for  c in command.chars() {
                            chars.push(c);
                            print!("{c}");
                        }
                        std::io::Write::flush(&mut stdout()).unwrap();
                    }
                }
            }
            Key::Backspace => {
                if !chars.is_empty() {
                    chars.pop();
                    print!("\x08 \x08");
                    std::io::Write::flush(&mut stdout()).unwrap();
                }
            }
            Key::Enter => {
                let input = chars.iter().collect::<String>();
                if input.is_empty() {continue}
                println!();
                return input
            }
            _ => {

            }
        }
    }
}


use std::{process};
use std::process::{Command, Stdio};
pub fn handle_exit() -> Result<(), Box<dyn std::error::Error>> {
    process::exit(1);
}
pub fn handle_clear() -> Result<(), Box<dyn std::error::Error>> {
    clear_screen::clear();
    Ok(())
}
pub fn handle_help() -> Result<(), Box<dyn std::error::Error>> {
    println!("couldnt  be bothered tbh");
    Ok(())
}
pub fn handle_external_command(command: &str, args: &[&str]) {
    let output = Command::new(command.to_string())
        .args(args)
        .env("TERM", "xterm-256color")
        .stdout(Stdio::piped())
        .output();
    match output {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("{}", stdout);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("{stderr}");
            }
        }
        Err(_error) => {
            println!("dksh: {} not found :(", command);
        }
    }
}

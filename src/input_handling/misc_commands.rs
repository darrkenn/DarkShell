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
            let mut parsed_err: Vec<&str> = stderr.trim().split(' ').collect();
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

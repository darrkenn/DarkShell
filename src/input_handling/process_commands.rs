use sysinfo::System;

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
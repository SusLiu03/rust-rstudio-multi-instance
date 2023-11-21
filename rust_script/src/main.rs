use std::env;
use std::process::Command;

fn main() {
    // Set the RANACAPA_PORT environmental variable
    env::set_var("RANACAPA_PORT", "6407");  // Set your desired port number

    // Specify the path to R executable
    let r_executable_path = "/usr/bin/R";  // Adjust this path based on your actual setup

    // Run the Shiny app using R
    let command_r = Command::new(&r_executable_path)
        .arg("-e")
        .arg("library(ranacapa); ranacapa::runRanacapaApp()");
    match command_r.status() {
        Ok(status) if status.success() => {
            println!("Shiny app started successfully. Visit http://localhost:6407/ranacapa to access the app.");
        },
        Ok(status) => eprintln!("Error starting Shiny app: {:?}", status),
        Err(e) => eprintln!("Error starting Shiny app: {}", e),
    }
}
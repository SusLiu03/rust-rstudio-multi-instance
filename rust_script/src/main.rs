// use std::env;
// use std::process::Command;
// use std::thread;

// fn main() {
//     // Set the RANACAPA_PORT environmental variable
//     let handle = thread::spawn(|| {
//         env::set_var("RANACAPA_PORT", "6407");  // Set your desired port number

//         // Specify the path to R executable
//         let r_executable_path = "/usr/bin/R";  // Adjust this path based on your actual setup

//         // Run the Shiny app using R
//         let mut binding = Command::new(&r_executable_path);
//         let command_r = binding.arg("-e").arg("library(ranacapa); ranacapa::runRanacapaApp()");
//         match command_r.status() {
//             Ok(status) if status.success() => {
//                 println!("Shiny app started successfully. Visit http://localhost:6407/ranacapa to access the app.");
//             },
//             Ok(status) => eprintln!("Error starting Shiny app: {:?}", status),
//             Err(e) => eprintln!("Error starting Shiny app: {}", e),
//         }
//     });

//     // Wait for the spawned thread to finish
//     handle.join().unwrap();
// }

use std::env;
use std::process::Command;
use std::thread;

fn main() {
    // Specify the path to R executable
    let r_executable_path = "/usr/bin/R"; // Adjust this path based on your actual setup

    // Generate a comma-separated list of port numbers
    let port_numbers: Vec<String> = (6407..6416).map(|port| port.to_string()).collect();
    let ports_string = port_numbers.join(",");

    // Update NGINX configuration with the new port numbers
    let nginx_command = format!("sudo nginx -s reload -g \"set $thread_ports {};\"", ports_string);

    // Execute the NGINX reload command
    Command::new("sh")
        .arg("-c")
        .arg(&nginx_command)
        .spawn()
        .expect("Failed to reload NGINX configuration");

    // Create 10 threads, each with a different port number
    let threads: Vec<_> = (6407..6416)
        .map(|port| {
            let r_executable_path_clone = r_executable_path.to_string();
            thread::spawn(move || {
                // Set the RANACAPA_PORT environmental variable
                env::set_var("RANACAPA_PORT", &port.to_string());

                // Run the Shiny app using R with the specific port
                let mut binding = Command::new(&r_executable_path_clone);
                let command_r = binding.arg("-e").arg("library(ranacapa); ranacapa::runRanacapaApp()");
                match command_r.status() {
                    Ok(status) if status.success() => {
                        println!("Shiny app started successfully on port {}.", port);
                    }
                    Ok(status) => eprintln!("Error starting Shiny app on port {}: {:?}", port, status),
                    Err(e) => eprintln!("Error starting Shiny app on port {}: {}", port, e),
                }
            })
        })
        .collect();

    // Wait for all threads to finish
    for thread in threads {
        thread.join().unwrap();
    }
}

use std::env;
use std::process::Command;
use std::thread;

fn main() {
    // Set the RANACAPA_PORT environmental variable
    let handle = thread::spawn(|| {
        env::set_var("RANACAPA_PORT", "6407");  // Set your desired port number

        // Specify the path to R executable
        let r_executable_path = "/usr/bin/R";  // Adjust this path based on your actual setup

        // Run the Shiny app using R
        let mut binding = Command::new(&r_executable_path);
        let command_r = binding.arg("-e").arg("library(ranacapa); ranacapa::runRanacapaApp()");
        match command_r.status() {
            Ok(status) if status.success() => {
                println!("Shiny app started successfully. Visit http://localhost:6407/ranacapa to access the app.");
            },
            Ok(status) => eprintln!("Error starting Shiny app: {:?}", status),
            Err(e) => eprintln!("Error starting Shiny app: {}", e),
        }
    });

    // Wait for the spawned thread to finish
    handle.join().unwrap();
}

// use std::env;
// use std::process::Command;
// use std::thread;

// fn main() {
//     // Set the RANACAPA_PORT environmental variable
//     // Specify the path to R executable
//     let r_executable_path = "/usr/bin/R"; // Adjust this path based on your actual setup

//     // Create 10 threads, each with a different port number
//     let threads: Vec<_> = (0..1)
//         .map(|i| {
//             let port = 6407 + i;
//             env::set_var("RANACAPA_PORT", &port.to_string()); // Set your desired port number
//             thread::spawn(move || {
//                 // Run the Shiny app using R with the specific port
//             let mut binding = Command::new(&r_executable_path);
//             let command_r = binding.arg("-e").arg("library(ranacapa); ranacapa::runRanacapaApp()");
//             match command_r.status() {
//                 Ok(status) if status.success() => {
//                     println!("Shiny app started successfully. Visit http://localhost:6407/ranacapa to access the app.");
//                 },
//                 Ok(status) => eprintln!("Error starting Shiny app: {:?}", status),
//                 Err(e) => eprintln!("Error starting Shiny app: {}", e),
//                 }
//             })
//         })
//         .collect();

//     // Wait for all threads to finish
//     for thread in threads {
//         thread.join().unwrap();
//     }
// }
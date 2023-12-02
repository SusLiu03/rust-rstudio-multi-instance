use std::env;
use std::process::Command;
use std::thread;
use std::sync::mpsc;

fn main() {
    let r_executable_path = "/usr/bin/R"; // R executable path

    // Create a channel for communication between threads
    let (tx, rx) = mpsc::channel();

    // Create 20 threads, each with a different port number
    let threads: Vec<_> = (0..20)
        .map(|i| {
            let port = 6407 + i;
            let tx = tx.clone();

            thread::spawn(move || {
                // Run the Shiny app using R with the specific port
                let mut binding = Command::new(&r_executable_path);
                let command_r = binding.arg("-e").arg("library(ranacapa); ranacapa::runRanacapaApp()");

                match command_r.status() {
                    Ok(status) if status.success() => {
                        tx.send(format!("Shiny app started successfully on port {}", port)).unwrap();
                    },
                    Ok(status) => eprintln!("Error starting Shiny app: {:?}", status),
                    Err(e) => eprintln!("Error starting Shiny app: {}", e),
                }
            })
        })
        .collect();

    // Wait for all threads to finish and collect messages
    for thread in threads {
        thread.join().unwrap();
    }

    // Collect messages from threads
    for received in rx {
        println!("{}", received);
    }
}


// use std::env;
// use std::process::Command;
// use std::thread;

// fn main() {

//     let r_executable_path = "/usr/bin/R"; // R executable path

//     // Create 20 threads, each with a different port number
//     let threads: Vec<_> = (0..20)
//         .map(|i| {
//             let port = 6407 + i;
            
//             env::set_var("RANACAPA_PORT", &port.to_string()); // Set your desired port number
//             thread::spawn(move || {
//                 // Run the Shiny app using R with the specific port
//             let mut binding = Command::new(&r_executable_path);
//             let command_r = binding.arg("-e").arg("library(ranacapa); ranacapa::runRanacapaApp()");
//             match command_r.status() {
//                 Ok(status) if status.success() => {
//                     println!("Shiny app started successfully {}", port);
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
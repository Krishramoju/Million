
//! neurokernel/src/shell.rs

use std::io::{self, Write};

pub fn start() {
    println!("[SHELL] Launching NeuroShell CLI");
    let prompt = "neuro$ ";
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        handle_command(input.trim());
    }
}

fn handle_command(cmd: &str) {
    match cmd {
        "help" => println!("Available commands: help, exit"),
        "exit" => {
            println!("Exiting shell...");
            std::process::exit(0);
        }
        _ => println!("Unknown command: {}", cmd),
    }
}

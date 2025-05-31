
//! neurokernel/src/shell.rs

use std::io::{self, Write};
use std::process::Command;
use std::fs;

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
        "help" => println!("Available commands: help, exit, clear, date, whoami, ls, echo <text>, cat <file>, mkdir <dir>"),
        "exit" => {
            println!("Exiting shell...");
            std::process::exit(0);
        }
        "clear" => {
            print!("\x1B[2J\x1B[1;1H");
        }
        "date" => {
            let _ = Command::new("date").status();
        }
        "whoami" => {
            let _ = Command::new("whoami").status();
        }
        "ls" => {
            let _ = Command::new("ls").status();
        }
        _ if cmd.starts_with("echo ") => {
            let output = cmd.strip_prefix("echo ").unwrap();
            println!("{}", output);
        }
        _ if cmd.starts_with("cat ") => {
            let filename = cmd.strip_prefix("cat ").unwrap();
            match fs::read_to_string(filename) {
                Ok(content) => println!("{}", content),
                Err(e) => println!("Error reading file: {}", e),
            }
        }
        _ if cmd.starts_with("mkdir ") => {
            let dirname = cmd.strip_prefix("mkdir ").unwrap();
            match fs::create_dir(dirname) {
                Ok(_) => println!("Directory '{}' created", dirname),
                Err(e) => println!("Error creating directory: {}", e),
            }
        }
        _ => println!("Unknown command: {}", cmd),
    }
}

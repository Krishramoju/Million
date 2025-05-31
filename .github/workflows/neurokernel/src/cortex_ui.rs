//! neurokernel/src/cortex_ui.rs
//! Terminal UI Layer for Neurokernel using Crossterm

use std::io::{stdout, Write};
use crossterm::{ExecutableCommand, terminal, cursor, style::{self, Colorize}};

pub fn launch_ui() {
    let mut stdout = stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();

    println!("{}", "🧠 Welcome to CortexUI".blue().bold());
    println!("{}", "─────────────────────────────");
    println!("{}", "[F1] Shell        [F2] LLM Console");
    println!("{}", "[F3] File System  [F4] NeuroScript");
    println!("{}", "[ESC] Exit");

    println!("\nPress a function key to proceed...");

    wait_for_input();
}

fn wait_for_input() {
    use crossterm::event::{read, Event, KeyCode};

    loop {
        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Esc => {
                    println!("Exiting CortexUI...");
                    break;
                }
                KeyCode::F(1) => println!("Launching Shell..."),
                KeyCode::F(2) => println!("Opening LLM Console..."),
                KeyCode::F(3) => println!("Accessing File System..."),
                KeyCode::F(4) => println!("Running NeuroScript..."),
                _ => println!("Invalid key. Try [F1]–[F4] or [ESC]"),
            }
        }
    }
}

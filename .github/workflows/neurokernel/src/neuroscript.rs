//! neurokernel/src/neuroscript.rs
//! Language: NeuroScript (Natural CLI commands)

use std::collections::HashMap;
use std::fs;

pub struct NeuroScriptEngine {
    commands: HashMap<&'static str, fn(&str)>,
}

impl NeuroScriptEngine {
    pub fn new() -> Self {
        let mut engine = NeuroScriptEngine {
            commands: HashMap::new(),
        };
        engine.register("say", |args| println!("{}", args));
        engine.register("make file", |args| {
            if let Err(e) = fs::write(args, "") {
                println!("Error: {}", e);
            } else {
                println!("File '{}' created", args);
            }
        });
        engine.register("read file", |args| {
            match fs::read_to_string(args) {
                Ok(content) => println!("{}", content),
                Err(e) => println!("Error: {}", e),
            }
        });
        engine.register("make folder", |args| {
            match fs::create_dir(args) {
                Ok(_) => println!("Folder '{}' created", args),
                Err(e) => println!("Error: {}", e),
            }
        });
        engine.register("delete", |args| {
            match fs::remove_file(args) {
                Ok(_) => println!("Deleted '{}'", args),
                Err(e) => println!("Error: {}", e),
            }
        });
        engine.register("list", |_| {
            match fs::read_dir(".") {
                Ok(entries) => {
                    for entry in entries.flatten() {
                        println!("{}", entry.path().display());
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
        });
        engine.register("rename", |args| {
            let parts: Vec<&str> = args.split(" to ").collect();
            if parts.len() == 2 {
                match fs::rename(parts[0], parts[1]) {
                    Ok(_) => println!("Renamed '{}' to '{}'", parts[0], parts[1]),
                    Err(e) => println!("Error: {}", e),
                }
            } else {
                println!("Usage: rename <old> to <new>");
            }
        });
        engine
    }

    fn register(&mut self, command: &'static str, action: fn(&str)) {
        self.commands.insert(command, action);
    }

    pub fn interpret(&self, input: &str) {
        for (cmd, action) in &self.commands {
            if input.starts_with(cmd) {
                let args = input.strip_prefix(cmd).unwrap().trim();
                action(args);
                return;
            }
        }
        println!("Unknown NeuroScript command: {}", input);
    }
}

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub enum DebugCommand {
    Step,
    Continue,
    Break(u32),
    Inspect(String),
    Watch(String),
    Quit,
}

#[derive(Debug)]
pub struct Debugger {
    pub breakpoints: HashSet<u32>,
    pub watch_list: HashSet<String>,
    pub current_line: u32,
    pub variables: HashMap<String, String>,
    pub halted: bool,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            breakpoints: HashSet::new(),
            watch_list: HashSet::new(),
            current_line: 0,
            variables: HashMap::new(),
            halted: false,
        }
    }

    pub fn set_breakpoint(&mut self, line: u32) {
        self.breakpoints.insert(line);
        println!("🔖 Breakpoint set at line {}", line);
    }

    pub fn remove_breakpoint(&mut self, line: u32) {
        self.breakpoints.remove(&line);
        println!("🗑️ Breakpoint removed from line {}", line);
    }

    pub fn step(&mut self) {
        self.current_line += 1;
        self.halted = true;
        println!("⏭️ Step into line {}", self.current_line);
    }

    pub fn continue_exec(&mut self) {
        self.halted = false;
        println!("▶️ Continuing execution...");
    }

    pub fn should_break(&self) -> bool {
        self.breakpoints.contains(&self.current_line)
    }

    pub fn inspect_variable(&self, var_name: &str) {
        if let Some(value) = self.variables.get(var_name) {
            println!("🔍 {} = {}", var_name, value);
        } else {
            println!("❓ Variable '{}' not found.", var_name);
        }
    }

    pub fn watch_variable(&mut self, var_name: &str) {
        self.watch_list.insert(var_name.to_string());
        println!("👁️ Watching variable '{}'", var_name);
    }

    pub fn update_variable(&mut self, var_name: &str, value: &str) {
        self.variables.insert(var_name.to_string(), value.to_string());
        if self.watch_list.contains(var_name) {
            println!("📡 {} changed: {}", var_name, value);
        }
    }

    pub fn tick(&mut self) {
        if self.should_break() {
            self.halted = true;
            println!("🛑 Hit breakpoint at line {}", self.current_line);
        }
    }

    pub fn interpret_command(&mut self, command: DebugCommand) {
        match command {
            DebugCommand::Step => self.step(),
            DebugCommand::Continue => self.continue_exec(),
            DebugCommand::Break(line) => self.set_breakpoint(line),
            DebugCommand::Inspect(var) => self.inspect_variable(&var),
            DebugCommand::Watch(var) => self.watch_variable(&var),
            DebugCommand::Quit => {
                println!("👋 Exiting debugger.");
                std::process::exit(0);
            }
        }
    }
}

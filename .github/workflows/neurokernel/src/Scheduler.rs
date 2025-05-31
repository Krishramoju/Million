//! neurokernel/src/scheduler.rs
//! Task and thread scheduler simulation for multitasking in Neurokernel OS.

use std::collections::VecDeque;
use std::thread;
use std::time::{Duration, Instant};

pub struct Task {
    pub id: usize,
    pub name: String,
    pub duration_ms: u64,
}

pub struct Scheduler {
    queue: VecDeque<Task>,
    current_task: Option<Task>,
    next_id: usize,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            queue: VecDeque::new(),
            current_task: None,
            next_id: 0,
        }
    }

    pub fn add_task(&mut self, name: &str, duration_ms: u64) -> usize {
        let task = Task {
            id: self.next_id,
            name: name.to_string(),
            duration_ms,
        };
        self.next_id += 1;
        self.queue.push_back(task);
        self.next_id - 1
    }

    pub fn run(&mut self) {
        println!("🚦 Starting scheduler...");
        while !self.queue.is_empty() {
            let task = self.queue.pop_front().unwrap();
            self.current_task = Some(task);
            println!("⏳ Running task: {}", self.current_task.as_ref().unwrap().name);
            let start = Instant::now();
            thread::sleep(Duration::from_millis(self.current_task.as_ref().unwrap().duration_ms));
            let elapsed = start.elapsed();
            println!("✅ Task '{}' completed in {:?}.", self.current_task.as_ref().unwrap().name, elapsed);
            self.current_task = None;
        }
        println!("🏁 All scheduled tasks completed.");
    }
}

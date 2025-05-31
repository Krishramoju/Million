//! neurokernel/src/memory_lane.rs
//! Visual history of user activity with basic analytics

use std::collections::HashMap;
use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct SearchRecord {
    pub query: String,
    pub timestamp: DateTime<Local>,
}

#[derive(Default)]
pub struct MemoryLane {
    history: Vec<SearchRecord>,
}

impl MemoryLane {
    pub fn new() -> Self {
        MemoryLane {
            history: Vec::new(),
        }
    }

    pub fn log_query(&mut self, query: &str) {
        self.history.push(SearchRecord {
            query: query.to_string(),
            timestamp: Local::now(),
        });
    }

    pub fn show_timeline(&self) {
        println!("\n📜 Your Memory Lane Timeline:\n---------------------------");
        for record in &self.history {
            println!("{} — {}", record.timestamp.format("%Y-%m-%d %H:%M:%S"), record.query);
        }
    }

    pub fn visualize_keywords(&self) {
        let mut freq: HashMap<String, usize> = HashMap::new();
        for record in &self.history {
            for word in record.query.split_whitespace() {
                *freq.entry(word.to_string()).or_insert(0) += 1;
            }
        }

        println!("\n📊 Keyword Frequency:");
        for (word, count) in freq {
            println!("{:<15} | {:<3} {}", word, count, "█".repeat(count));
        }
    }
}

//! neurokernel/src/notifier.rs
//! Notification system for user alerts, system messages, and async events.

use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub enum NotificationLevel {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub timestamp: u64,
    pub message: String,
    pub level: NotificationLevel,
}

impl Notification {
    pub fn new(message: &str, level: NotificationLevel) -> Self {
        Self {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            message: message.to_string(),
            level,
        }
    }

    pub fn display(&self) {
        let level_str = match self.level {
            NotificationLevel::Info => "ℹ️ INFO",
            NotificationLevel::Warning => "⚠️ WARNING",
            NotificationLevel::Critical => "🛑 CRITICAL",
        };

        println!(
            "[{} | {}] {}",
            self.timestamp, level_str, self.message
        );
    }
}

pub struct Notifier {
    queue: VecDeque<Notification>,
}

impl Notifier {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn push(&mut self, message: &str, level: NotificationLevel) {
        let note = Notification::new(message, level);
        note.display();
        self.queue.push_back(note);
    }

    pub fn flush(&mut self) {
        println!("🔔 Flushing all notifications:");
        while let Some(note) = self.queue.pop_front() {
            note.display();
        }
    }

    pub fn latest(&self) -> Option<&Notification> {
        self.queue.back()
    }
}

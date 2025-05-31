//! neurokernel/src/ml_monitor.rs
//! Monitors and visualizes real-time LLM activity, usage, and performance metrics.

use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct LLMInvocation {
    pub prompt_tokens: usize,
    pub response_tokens: usize,
    pub duration: Duration,
    pub model: String,
    pub timestamp: Instant,
}

#[derive(Default)]
pub struct MLMonitor {
    invocations: Vec<LLMInvocation>,
    model_stats: HashMap<String, usize>, // model_name -> total invocations
}

impl MLMonitor {
    pub fn new() -> Self {
        Self {
            invocations: Vec::new(),
            model_stats: HashMap::new(),
        }
    }

    pub fn record_invocation(
        &mut self,
        model: &str,
        prompt_tokens: usize,
        response_tokens: usize,
        duration: Duration,
    ) {
        let invocation = LLMInvocation {
            model: model.to_string(),
            prompt_tokens,
            response_tokens,
            duration,
            timestamp: Instant::now(),
        };

        self.invocations.push(invocation);
        *self.model_stats.entry(model.to_string()).or_insert(0) += 1;
    }

    pub fn total_invocations(&self) -> usize {
        self.invocations.len()
    }

    pub fn most_used_model(&self) -> Option<(&String, &usize)> {
        self.model_stats.iter().max_by_key(|entry| entry.1)
    }

    pub fn average_response_time(&self) -> Option<Duration> {
        if self.invocations.is_empty() {
            return None;
        }
        let total_duration: Duration = self
            .invocations
            .iter()
            .map(|i| i.duration)
            .sum();

        Some(total_duration / self.invocations.len() as u32)
    }

    pub fn show_summary(&self) {
        println!("📊 LLM Usage Summary:");
        println!("  Total Invocations: {}", self.total_invocations());

        if let Some(avg) = self.average_response_time() {
            println!("  Avg. Response Time: {:.2?}", avg);
        }

        if let Some((model, count)) = self.most_used_model() {
            println!("  Most Used Model: {} ({} times)", model, count);
        }
    }

    pub fn debug_log(&self) {
        println!("🔍 Full Invocation Log:");
        for (i, inv) in self.invocations.iter().enumerate() {
            println!(
                "  #{} [{}]: {} prompt tokens, {} response tokens, {:.2?}, model: {}",
                i + 1,
                inv.timestamp.elapsed().as_secs(),
                inv.prompt_tokens,
                inv.response_tokens,
                inv.duration,
                inv.model
            );
        }
    }
}

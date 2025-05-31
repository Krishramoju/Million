//! neurokernel/src/dev_tools.rs
//! Developer utility functions for debugging, profiling, and diagnostics

use std::fs;
use std::time::{Duration, Instant};
use std::thread;

pub fn debug_log(file: &str, message: &str) {
    let entry = format!("[DEBUG] {}\n", message);
    fs::write(file, entry).unwrap_or_else(|_| println!("Failed to write to log file."));
}

pub fn simulate_load(task: fn(), name: &str) {
    println!("🔧 Simulating task: {}", name);
    let start = Instant::now();
    task();
    let duration = start.elapsed();
    println!("⏱️  Completed in: {:?}\n", duration);
}

pub fn run_benchmark(task: fn(), iterations: usize) {
    let mut total = Duration::new(0, 0);
    println!("📈 Benchmarking over {} iterations...", iterations);
    for _ in 0..iterations {
        let start = Instant::now();
        task();
        total += start.elapsed();
    }
    let avg = total / iterations as u32;
    println!("⚙️  Average execution time: {:?}\n", avg);
}

pub fn crash_test() {
    println!("💥 Running crash test (panic)...");
    panic!("This is a simulated crash for testing");
}

pub fn memory_probe() {
    println!("🧠 Memory probe running... (placeholder)");
    // In real OS: attach to allocator stats or /proc
    thread::sleep(Duration::from_millis(500));
    println!("✅ Memory check complete\n");
}

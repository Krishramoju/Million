use std::thread;
use std::time::{Duration, Instant};
use std::io::{self, Write};

// Simulate LED feedback
fn led_feedback(correct: bool) {
    if correct {
        println!("🟢 LED: Correct Answer!");
    } else {
        println!("🔴 LED: Wrong Answer!");
    }
}

// Simulated question logic
fn ask_question(question: &str, answer: &str) -> bool {
    println!("❓ {}", question);
    print!("Your answer: ");
    io::stdout().flush().unwrap();

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let user_input = user_input.trim().to_lowercase();

    user_input == answer.to_lowercase()
}

// Main microcontroller simulation
fn microcontroller_brain_teaser() {
    let questions = vec![
        ("What has keys but can’t open locks?", "keyboard"),
        ("What has hands but can’t clap?", "clock"),
        ("What runs but never walks?", "water"),
    ];

    println!("🎮 Brain Teaser Starting!");
    let start_time = Instant::now();

    for (question, answer) in &questions {
        let correct = ask_question(question, answer);
        led_feedback(correct);
        thread::sleep(Duration::from_secs(1));
    }

    let duration = Instant::now() - start_time;
    println!("✅ Game over! Time taken: {:.2?}", duration);
}

fn main() {
    println!("🔌 Microcontroller Ready. Press the 'Start' button (Enter key) to begin Brain Teaser...");
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();

    microcontroller_brain_teaser();
}

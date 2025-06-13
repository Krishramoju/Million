use std::thread;
use std::time::Duration;

// Mock input types
enum Input {
    ButtonPressed,
    VoiceDetected(String),
}

// Simulate LED feedback (terminal print for demonstration)
fn led_feedback(success: bool) {
    if success {
        println!("🟢 LED ON: Chatbot activated.");
    } else {
        println!("🔴 LED BLINK: Unrecognized input.");
    }
}

// Simulated microcontroller loop
fn microcontroller_loop() {
    let simulated_inputs = vec![
        Input::ButtonPressed,
        Input::VoiceDetected(String::from("hello bot")),
        Input::VoiceDetected(String::from("unknown phrase")),
    ];

    for input in simulated_inputs {
        match input {
            Input::ButtonPressed => {
                println!("🔘 Button pressed. Activating chatbot...");
                led_feedback(true);
                send_to_chatbot("User activated chatbot via button.");
            }
            Input::VoiceDetected(phrase) => {
                if phrase.to_lowercase().contains("hello") {
                    println!("🎤 Voice detected: '{}'", phrase);
                    led_feedback(true);
                    send_to_chatbot(&phrase);
                } else {
                    println!("🛑 Voice command not recognized: '{}'", phrase);
                    led_feedback(false);
                }
            }
        }
        thread::sleep(Duration::from_secs(2));
    }
}

// Mock chatbot communication
fn send_to_chatbot(message: &str) {
    println!("📡 Sending to chatbot API: '{}'", message);
}

fn main() {
    println!("🧠 Microcontroller booted. Listening for input...");
    microcontroller_loop();
}

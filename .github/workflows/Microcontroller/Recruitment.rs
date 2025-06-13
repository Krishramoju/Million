use std::thread;
use std::time::Duration;

// Simulate sensor data
enum InputEvent {
    FingerprintScan(bool),   // true = match
    PhotoCapture,
    FormFilled(bool),        // true = complete form
}

// Simulate LED feedback
fn led_feedback(success: bool) {
    if success {
        println!("🟢 LED: Data accepted and stored.");
    } else {
        println!("🔴 LED: Invalid input or form incomplete.");
    }
}

// Mock storage handler
fn store_to_buffer(data: &str) {
    println!("💾 Storing securely to buffer: {}", data);
    // In real system, encrypt and store to EEPROM/SD card here
}

// Simulated microcontroller routine
fn microcontroller_recruitment() {
    let simulated_inputs = vec![
        InputEvent::FingerprintScan(true),
        InputEvent::PhotoCapture,
        InputEvent::FormFilled(true),
    ];

    for input in simulated_inputs {
        match input {
            InputEvent::FingerprintScan(valid) => {
                if valid {
                    println!("🔐 Fingerprint matched.");
                    store_to_buffer("Fingerprint verified.");
                    led_feedback(true);
                } else {
                    println!("🛑 Fingerprint mismatch.");
                    led_feedback(false);
                }
            }
            InputEvent::PhotoCapture => {
                println!("📸 Capturing photo...");
                store_to_buffer("Photo ID captured.");
                led_feedback(true);
            }
            InputEvent::FormFilled(complete) => {
                if complete {
                    println!("📝 Form filled successfully.");
                    store_to_buffer("Form data stored.");
                    led_feedback(true);
                } else {
                    println!("⚠️ Form incomplete.");
                    led_feedback(false);
                }
            }
        }
        thread::sleep(Duration::from_secs(2));
    }

    println!("✅ All data securely stored. Ready for cloud sync.");
}

fn main() {
    println!("🔌 Microcontroller booting for Recruitment Form...");
    microcontroller_recruitment();
}

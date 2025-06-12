use sysinfo::{System, SystemExt, ProcessExt, CpuExt, NetworkExt};
use chrono::Local;

fn main() {
    let now = Local::now();
    println!("🔍 Neuro OS - Advanced Diagnostic\nTimestamp: {}\n", now);

    let mut sys = System::new_all();
    sys.refresh_all();

    let mut issues_found = false;

    // CPU Overuse Detection
    println!("🧠 Checking for CPU Overuse...");
    for (pid, proc) in sys.processes() {
        if proc.cpu_usage() > 50.0 {
            issues_found = true;
            println!(" - ⚠️ Process '{}' (PID {}) is using {:.2}% CPU", proc.name(), pid, proc.cpu_usage());
        }
    }

    // RAM Overuse Detection
    println!("\n📛 Checking for RAM Overuse...");
    for (pid, proc) in sys.processes() {
        let mem_usage = proc.memory() as f64 / 1024.0; // MB
        if mem_usage > 500.0 {
            issues_found = true;
            println!(" - ⚠️ Process '{}' (PID {}) is using {:.2} MB RAM", proc.name(), pid, mem_usage);
        }
    }

    // Network Check
    println!("\n🌐 Checking Network Interfaces...");
    if sys.networks().is_empty() {
        issues_found = true;
        println!(" - 🚫 No active network interfaces found!");
    } else {
        println!(" - ✅ Network interfaces are active");
    }

    // Dummy check for frozen services (simulated logic)
    println!("\n⚙️ Simulating Service Status...");
    let simulated_services = vec![("Camera", true), ("Bluetooth", false), ("FileSync", true)];
    for (service, is_running) in simulated_services {
        if !is_running {
            issues_found = true;
            println!(" - ❌ Service '{}' is unresponsive", service);
        }
    }

    // Summary
    if issues_found {
        println!("\n🔧 Suggested Actions:");
        println!(" - Kill or restart high-CPU/RAM processes");
        println!(" - Reconnect network or check adapters");
        println!(" - Restart unresponsive services manually or via Neuro OS assistant");
    } else {
        println!("\n✅ No critical issues found. System is healthy.");
    }
}

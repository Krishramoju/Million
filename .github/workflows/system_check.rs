use sysinfo::{System, SystemExt, DiskExt, CpuExt, NetworkExt};
use chrono::Local;

fn main() {
    let now = Local::now();
    println!("🧠 Neuro OS - System Check\nTimestamp: {}\n", now);

    let mut sys = System::new_all();
    sys.refresh_all();

    // CPU Info
    println!("🖥️ CPU:");
    for (i, cpu) in sys.cpus().iter().enumerate() {
        println!(" - Core {}: {:.2}% usage", i, cpu.cpu_usage());
    }

    // Memory Info
    println!("\n🧠 Memory:");
    let total_mem = sys.total_memory() as f64 / 1024.0;
    let used_mem = sys.used_memory() as f64 / 1024.0;
    println!(" - Total Memory: {:.2} MB", total_mem);
    println!(" - Used Memory : {:.2} MB", used_mem);

    // Disk Info
    println!("\n💾 Disk:");
    for disk in sys.disks() {
        let total = disk.total_space() as f64 / 1_073_741_824.0;
        let available = disk.available_space() as f64 / 1_073_741_824.0;
        println!(
            " - {}: {:.2} GB total, {:.2} GB available",
            disk.name().to_string_lossy(),
            total,
            available
        );
    }

    // Network Info
    println!("\n🌐 Network Interfaces:");
    for (interface_name, data) in sys.networks() {
        println!(
            " - {}: RX {:.2} KB | TX {:.2} KB",
            interface_name,
            data.received() as f64 / 1024.0,
            data.transmitted() as f64 / 1024.0
        );
    }

    println!("\n✅ System check completed.");
}

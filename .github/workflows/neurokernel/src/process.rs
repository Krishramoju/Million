
//! neurokernel/src/process.rs

pub fn init() {
    println!("[PROC] Process manager started");
    create_idle_process();
    create_init_process();
}

fn create_idle_process() {
    println!("[PROC] Idle process created with PID 0");
    // Simulated: minimal background process
}

fn create_init_process() {
    println!("[PROC] Init process created with PID 1");
    // Simulated: first user process that launches shell
}

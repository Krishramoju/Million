pub fn evaluate_class_noise(db: f32) -> String {
    if db > 70.0 {
        "High noise: Consider classroom discipline strategies.".into()
    } else {
        "Noise levels acceptable.".into()
    }
}

pub fn check_attendance(present: u32, total: u32) -> String {
    let percentage = (present as f32 / total as f32) * 100.0;
    if percentage < 75.0 {
        format!("Attendance low ({:.2}%): Follow-up needed.", percentage)
    } else {
        format!("Good attendance ({:.2}%).", percentage)
    }

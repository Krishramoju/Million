pub fn analyze_heart_rate(bpm: u32) -> String {
    if bpm < 50 {
        "Bradycardia detected: Monitor patient closely.".into()
    } else if bpm > 100 {
        "Tachycardia alert: Immediate medical review advised.".into()
    } else {
        "Heart rate normal.".into()
    }
}

pub fn monitor_oxygen_level(spo2: u8) -> String {
    if spo2 < 92 {
        "Oxygen level low: Administer O2 immediately.".into()
    } else {
        "Oxygen level within normal range.".into()
    }
}

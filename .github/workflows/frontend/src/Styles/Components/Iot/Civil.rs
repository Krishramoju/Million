// src/iot/civil_engineer.rs
pub fn monitor_vibrations(level: f32) -> String {
    if level > 5.0 {
        "Warning: Structural vibrations exceed safe limits!".into()
    } else {
        "Structure stable.".into()
    }
}

pub fn check_air_quality(pm25: f32) -> String {
    if pm25 > 100.0 {
        "High particulate matter: Recommend masks on site.".into()
    } else {
        "Air quality within safe limits.".into()
    }
}

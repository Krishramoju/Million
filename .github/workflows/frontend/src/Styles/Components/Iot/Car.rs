pub fn check_vehicle_speed(speed: f32) -> String {
    if speed > 80.0 {
        "Speed limit exceeded: Alert driver.".into()
    } else {
        "Speed within safe range.".into()
    }
}
pub fn track_fuel_level(fuel: f32) -> String {
    if fuel < 10.0 {
        "Low fuel warning: Refuel soon.".into()
    } else {
        "Fuel level sufficient.".into()
    }

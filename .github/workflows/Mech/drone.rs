// autonomous_drone.rs

use std::collections::HashMap;

pub struct DroneInput {
    pub altitude: f32,
    pub battery: f32,
    pub gps_lock: bool,
    pub obstacle_distance: f32,
}

#[derive(Debug)]
pub enum DroneAction {
    Ascend,
    Descend,
    Hover,
    ReturnHome,
    AvoidLeft,
    AvoidRight,
}

pub struct AutonomousDrone;

impl AutonomousDrone {
    pub fn new() -> Self { AutonomousDrone }

    pub fn evaluate(&self, input: &DroneInput) -> DroneAction {
        if input.battery < 20.0 {
            return DroneAction::ReturnHome;
        }
        if input.obstacle_distance < 1.0 {
            return DroneAction::AvoidLeft;
        }
        if input.altitude < 5.0 {
            return DroneAction::Ascend;
        }
        DroneAction::Hover
    }
}

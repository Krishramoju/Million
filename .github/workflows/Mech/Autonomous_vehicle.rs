// autonomous_vehicle.rs

use std::collections::HashMap;

pub struct VehicleInput {
    pub distance_front: f32,
    pub speed: f32,
    pub lane_offset: f32,
    pub traffic_light: Option<String>,
}

#[derive(Debug)]
pub enum VehicleAction {
    Accelerate,
    Brake,
    SteerLeft,
    SteerRight,
    Stop,
}

pub struct AutonomousVehicle;

impl AutonomousVehicle {
    pub fn new() -> Self { AutonomousVehicle }

    pub fn evaluate(&self, input: &VehicleInput) -> VehicleAction {
        if let Some(ref light) = input.traffic_light {
            if light == "red" { return VehicleAction::Stop; }
        }
        if input.distance_front < 2.0 {
            return VehicleAction::Brake;
        }
        if input.lane_offset.abs() > 0.3 {
            return if input.lane_offset > 0.0 {
                VehicleAction::SteerLeft
            } else {
                VehicleAction::SteerRight
            };
        }
        VehicleAction::Accelerate
    }
}

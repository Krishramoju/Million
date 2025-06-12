// autonomous_boat.rs

pub struct BoatInput {
    pub gps_signal: bool,
    pub battery: f32,
    pub wave_height: f32,
    pub obstacle_distance: f32,
}

#[derive(Debug)]
pub enum BoatAction {
    Sail,
    Anchor,
    ReturnDock,
    AvoidObstacle,
}

pub struct AutonomousBoat;

impl AutonomousBoat {
    pub fn new() -> Self { AutonomousBoat }

    pub fn evaluate(&self, input: &BoatInput) -> BoatAction {
        if input.battery < 20.0 || !input.gps_signal {
            return BoatAction::ReturnDock;
        }
        if input.obstacle_distance < 1.5 {
            return BoatAction::AvoidObstacle;
        }
        if input.wave_height > 2.0 {
            BoatAction::Anchor
        } else {
            BoatAction::Sail
        }
    }
}

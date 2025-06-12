// autonomous_robot.rs

pub struct RobotInput {
    pub proximity_front: f32,
    pub battery: f32,
    pub task_queue_full: bool,
}

#[derive(Debug)]
pub enum RobotAction {
    MoveForward,
    Turn,
    Stop,
    Recharge,
    StartTask,
}

pub struct AutonomousRobot;

impl AutonomousRobot {
    pub fn new() -> Self { AutonomousRobot }

    pub fn evaluate(&self, input: &RobotInput) -> RobotAction {
        if input.battery < 15.0 { return RobotAction::Recharge; }
        if input.proximity_front < 0.5 { return RobotAction::Turn; }
        if input.task_queue_full {
            RobotAction::StartTask
        } else {
            RobotAction::MoveForward
        }
    }
}

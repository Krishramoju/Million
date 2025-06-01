// backend/drivers/mod.rs

pub mod touchscreen;
pub mod camera;
pub mod sensors;

pub use touchscreen::{TouchscreenDriver, GenericTouchscreen, TouchPoint};
pub use camera::{CameraDriver, GenericCamera, CameraFrame};
pub use sensors::{SensorDriver, GenericSensor, SensorType, SensorData};

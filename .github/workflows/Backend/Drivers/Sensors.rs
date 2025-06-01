
// backend/drivers/sensors.rs

pub enum SensorType {
    Accelerometer,
    Gyroscope,
    Magnetometer,
    Proximity,
}

pub struct SensorData {
    pub sensor_type: SensorType,
    pub values: [f32; 3],
}

pub trait SensorDriver {
    fn initialize(&self) -> bool;
    fn read_data(&self) -> Option<SensorData>;
}

pub struct GenericSensor {
    sensor_type: SensorType,
}

impl GenericSensor {
    pub fn new(sensor_type: SensorType) -> Self {
        Self { sensor_type }
    }
}

impl SensorDriver for GenericSensor {
    fn initialize(&self) -> bool {
        println!("{:?} sensor initialized.", self.sensor_type);
        true
    }

    fn read_data(&self) -> Option<SensorData> {
        // Return dummy data
        Some(SensorData {
            sensor_type: self.sensor_type.clone(),
            values: [0.0, 0.0, 0.0],
        })
    }
}

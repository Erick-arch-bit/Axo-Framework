#[derive(Debug, Clone)]
pub struct SensorData {
    pub accelerometer: Option<(f32, f32, f32)>,
    pub gyroscope: Option<(f32, f32, f32)>,
    pub magnetometer: Option<(f32, f32, f32)>,
}

impl SensorData {
    pub fn current() -> Self {
        Self {
            accelerometer: None,
            gyroscope: None,
            magnetometer: None,
        }
    }
}

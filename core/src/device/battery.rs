#[derive(Debug, Clone)]
pub struct BatteryInfo {
    pub level: f32,
    pub is_charging: bool,
    pub health: String,
    pub temperature: f32,
    pub voltage: f32,
    pub capacity: f32,
}

impl BatteryInfo {
    pub fn current() -> Self {
        Self {
            level: 100.0,
            is_charging: true,
            health: "good".to_string(),
            temperature: 25.0,
            voltage: 3.7,
            capacity: 100.0,
        }
    }
}

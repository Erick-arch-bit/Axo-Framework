#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub os_name: String,
    pub os_version: String,
    pub device_model: String,
    pub screen_width: f32,
    pub screen_height: f32,
    pub is_mobile: bool,
    pub language: String,
    pub timezone: String,
    pub app_version: String,
}

impl DeviceInfo {
    pub fn current() -> Self {
        Self {
            os_name: std::env::consts::OS.to_string(),
            os_version: String::new(),
            device_model: String::new(),
            screen_width: 1024.0,
            screen_height: 768.0,
            is_mobile: false,
            language: "en".to_string(),
            timezone: "UTC".to_string(),
            app_version: "0.1.0".to_string(),
        }
    }
}

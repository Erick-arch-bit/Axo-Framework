#[derive(Debug, Clone)]
pub struct NetworkInfo {
    pub connected: bool,
    pub connection_type: String,
    pub ip_address: String,
    pub signal_strength: f32,
    pub wifi_ssid: Option<String>,
    pub is_metered: bool,
    pub downlink_mbps: f32,
}

impl NetworkInfo {
    pub fn current() -> Self {
        Self {
            connected: true,
            connection_type: "ethernet".to_string(),
            ip_address: "127.0.0.1".to_string(),
            signal_strength: 100.0,
            wifi_ssid: None,
            is_metered: false,
            downlink_mbps: 100.0,
        }
    }
}

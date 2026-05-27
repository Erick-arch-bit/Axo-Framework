#[derive(Debug, Clone)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f64,
    pub altitude: Option<f64>,
}

pub enum GeoError {
    PermissionDenied,
    Unavailable,
    Timeout,
}

pub fn get_current_position() -> Result<GeoLocation, GeoError> {
    Err(GeoError::Unavailable)
}

impl GeoLocation {
    pub fn mock() -> Self {
        Self {
            latitude: 19.4326,
            longitude: -99.1332,
            accuracy: 100.0,
            altitude: None,
        }
    }
}

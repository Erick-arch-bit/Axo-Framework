#[derive(Debug, Clone)]
pub struct DisplayInfo {
    pub width: f32,
    pub height: f32,
    pub dpi: f32,
    pub refresh_rate: f32,
    pub brightness: f32,
    pub orientation: String,
    pub color_gamut: String,
    pub hdr_supported: bool,
}

impl DisplayInfo {
    pub fn current() -> Self {
        Self {
            width: 1024.0,
            height: 768.0,
            dpi: 96.0,
            refresh_rate: 60.0,
            brightness: 80.0,
            orientation: "landscape".to_string(),
            color_gamut: "sRGB".to_string(),
            hdr_supported: false,
        }
    }
}

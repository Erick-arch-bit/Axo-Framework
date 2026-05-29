#[derive(Debug, Clone)]
pub struct Haptics;

#[derive(Debug, Clone, Copy)]
pub enum HapticFeedback {
    Light,
    Medium,
    Heavy,
    Selection,
    Success,
    Warning,
    Error,
    Click,
}

impl Haptics {
    pub fn vibrate(duration_ms: u64) {
        println!("[Haptics] Vibrate for {}ms", duration_ms);
    }

    pub fn impact(style: HapticFeedback) {
        let name = match style {
            HapticFeedback::Light => "light",
            HapticFeedback::Medium => "medium",
            HapticFeedback::Heavy => "heavy",
            HapticFeedback::Selection => "selection",
            HapticFeedback::Success => "success",
            HapticFeedback::Warning => "warning",
            HapticFeedback::Error => "error",
            HapticFeedback::Click => "click",
        };
        println!("[Haptics] Impact: {}", name);
    }

    pub fn notification(style: HapticFeedback) {
        let name = match style {
            HapticFeedback::Success => "success",
            HapticFeedback::Warning => "warning",
            HapticFeedback::Error => "error",
            _ => "unknown",
        };
        println!("[Haptics] Notification: {}", name);
    }

    pub fn is_supported() -> bool {
        true
    }
}

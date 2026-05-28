use crate::Platform;

pub struct DesktopPlatform;

impl Platform for DesktopPlatform {
    fn name(&self) -> &'static str {
        "Desktop"
    }

    fn init(&self) {
        println!("[Platform] Initializing desktop (Linux/macOS/Windows)");
    }

    fn run(&self) {
        axo_core::window::run_with_rects(Vec::new());
    }
}

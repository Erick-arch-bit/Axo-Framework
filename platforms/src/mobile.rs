use crate::Platform;

pub struct MobilePlatform;

impl Platform for MobilePlatform {
    fn name(&self) -> &'static str {
        "Mobile"
    }

    fn init(&self) {
        println!("[Platform] Initializing mobile runtime");
    }

    fn run(&self) {
        println!("[Platform] Mobile event loop started");
    }
}

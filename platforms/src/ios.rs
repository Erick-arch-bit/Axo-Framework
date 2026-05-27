use crate::Platform;

pub struct IosPlatform;

impl Platform for IosPlatform {
    fn name(&self) -> &'static str {
        "iOS"
    }

    fn init(&self) {
        println!("[Platform] Initializing iOS via UIKit");
    }

    fn run(&self) {
        println!("[Platform] iOS run loop");
    }
}

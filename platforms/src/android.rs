use crate::Platform;

pub struct AndroidPlatform;

impl Platform for AndroidPlatform {
    fn name(&self) -> &'static str {
        "Android"
    }

    fn init(&self) {
        println!("[Platform] Initializing Android via JNI");
    }

    fn run(&self) {
        println!("[Platform] Android activity loop");
    }
}

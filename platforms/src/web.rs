use crate::Platform;

pub struct WebPlatform;

impl Platform for WebPlatform {
    fn name(&self) -> &'static str {
        "Web (WASM)"
    }

    fn init(&self) {
        println!("[Platform] Initializing WebGL2/WebGPU canvas");
    }

    fn run(&self) {
        println!("[Platform] Web request animation frame loop");
    }
}

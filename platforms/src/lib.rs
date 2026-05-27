pub mod desktop;
pub mod mobile;

#[cfg(target_os = "android")]
pub mod android;
#[cfg(target_os = "ios")]
pub mod ios;
#[cfg(target_arch = "wasm32")]
pub mod web;

pub trait Platform {
    fn name(&self) -> &'static str;
    fn init(&self);
    fn run(&self);
}

pub mod watcher;
pub mod ws_server;

use std::sync::{Arc, Mutex};
use crate::renderer::Rect;

pub struct SharedRects {
    pub rects: Arc<Mutex<Vec<Rect>>>,
}

impl Default for SharedRects {
    fn default() -> Self {
        Self::new()
    }
}

impl SharedRects {
    pub fn new() -> Self {
        SharedRects { rects: Arc::new(Mutex::new(Vec::new())) }
    }
}

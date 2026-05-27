pub struct WsServer;

impl Default for WsServer {
    fn default() -> Self {
        Self::new()
    }
}

impl WsServer {
    pub fn new() -> Self {
        WsServer
    }

    pub fn listen(&self) {
        println!("[WebSocket] Server listening on port 9876");
    }

    pub fn broadcast(&self, script: &str) {
        println!("[WebSocket] Broadcasting Lua script ({} bytes)", script.len());
    }
}

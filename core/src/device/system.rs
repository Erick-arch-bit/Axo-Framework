use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub total_memory_mb: f64,
    pub free_memory_mb: f64,
    pub cpu_usage: f32,
    pub cpu_cores: u32,
    pub cpu_arch: String,
    pub kernel_version: String,
    pub uptime_seconds: f64,
    pub hostname: String,
    pub username: String,
    pub locale: String,
}

impl SystemInfo {
    pub fn current() -> Self {
        let uptime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        let cores = std::thread::available_parallelism()
            .map(|n| n.get() as u32)
            .unwrap_or(4);
        let hostname = std::fs::read_to_string("/proc/sys/kernel/hostname")
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "unknown".to_string());
        Self {
            total_memory_mb: 8192.0,
            free_memory_mb: 4096.0,
            cpu_usage: 25.0,
            cpu_cores: cores,
            cpu_arch: std::env::consts::ARCH.to_string(),
            kernel_version: std::env::consts::OS.to_string(),
            uptime_seconds: uptime,
            hostname,
            username: std::env::var("USER")
                .or_else(|_| std::env::var("USERNAME"))
                .unwrap_or_else(|_| "unknown".to_string()),
            locale: "en-US".to_string(),
        }
    }
}

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: u64,
    pub hostname: String,
    pub cpu_usage: f32,
    pub memory_total: u64,
    pub memory_used: u64,
    pub disk_total: u64,
    pub disk_used: u64,
}

impl SystemMetrics {
    pub fn new(hostname: String, cpu_usage: f32, memory_total: u64, memory_used: u64, disk_total: u64, disk_used: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        SystemMetrics {
            timestamp,
            hostname,
            cpu_usage,
            memory_total,
            memory_used,
            disk_total,
            disk_used,
        }
    }
}
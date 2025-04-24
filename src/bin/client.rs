use anyhow::Result;
use monitoring_system::metrics::SystemMetrics;
use std::time::Duration;
use std::thread;
use systemstat::{Platform, System};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time;

#[tokio::main]
async fn main() -> Result<()> {
    let server_addr = "127.0.0.1:8080";
    let hostname = gethostname::gethostname()
        .to_string_lossy()
        .to_string();
    
    println!("Connecting to server at {}", server_addr);
    let mut stream = TcpStream::connect(server_addr).await?;
    println!("Connected to server");

    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let system = System::new();
    
    // Collection interval (5 seconds)
    let interval = Duration::from_secs(5);
    
    loop {
        // Collect system metrics
        let metrics = collect_metrics(&system, &hostname)?;
        
        // Send metrics to server
        let json = serde_json::to_string(&metrics)? + "\n";
        writer.write_all(json.as_bytes()).await?;
        
        // Wait for acknowledgment
        let mut response = String::new();
        reader.read_line(&mut response).await?;
        println!("Server response: {}", response.trim());
        
        // Wait for the next collection interval
        time::sleep(interval).await;
    }
}

fn collect_metrics(system: &System, hostname: &str) -> Result<SystemMetrics> {
    // CPU usage - handle platforms differently
    let cpu_usage = match system.cpu_load_aggregate() {
        Ok(cpu) => {
            thread::sleep(Duration::from_secs(1));
            match cpu.done() {
                Ok(cpu_stats) => cpu_stats.user * 100.0,
                Err(_) => 0.0, // Default value if we can't get CPU stats
            }
        },
        Err(_) => 0.0, // Default value if not supported
    };
    
    // Memory usage - handle failures gracefully
    let (memory_total, memory_used) = match system.memory() {
        Ok(memory) => (memory.total.as_u64(), memory.total.as_u64() - memory.free.as_u64()),
        Err(_) => (0, 0), // Default values if not supported
    };
    
    // Disk usage - try different paths or fall back to defaults
    let (disk_total, disk_used) = match system.mount_at("/") {
        Ok(disk) => (disk.total.as_u64(), disk.total.as_u64() - disk.free.as_u64()),
        Err(_) => {
            // On macOS, try a different path
            match system.mount_at("/System/Volumes/Data") {
                Ok(disk) => (disk.total.as_u64(), disk.total.as_u64() - disk.free.as_u64()),
                Err(_) => (0, 0), // Default values if not supported
            }
        }
    };
    
    Ok(SystemMetrics::new(
        hostname.to_string(),
        cpu_usage,
        memory_total,
        memory_used,
        disk_total,
        disk_used,
    ))
}
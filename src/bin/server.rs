use monitoring_system::metrics::SystemMetrics;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
// Replace std::sync::Mutex with tokio::sync::Mutex
use tokio::sync::Mutex;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use anyhow::Result;

// Update the MetricsStorage type to use tokio's Mutex
type MetricsStorage = Arc<Mutex<HashMap<String, Vec<SystemMetrics>>>>;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    let metrics_storage: MetricsStorage = Arc::new(Mutex::new(HashMap::new()));
    let (shutdown_tx, _) = broadcast::channel::<()>(1);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New client connected: {}", addr);

        let metrics_storage = Arc::clone(&metrics_storage);

      
        let _shutdown_rx = shutdown_tx.subscribe();

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, addr, metrics_storage).await {
                eprintln!("Error handling client {}: {}", addr, e);
            }
        });
    }
}

async fn handle_client(
    mut socket: TcpStream,
    addr: SocketAddr,
    metrics_storage: MetricsStorage,
) -> Result<()> {
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;
        if bytes_read == 0 {
            println!("Client {} disconnected", addr);
            break;
        }

        if let Ok(metrics) = serde_json::from_str::<SystemMetrics>(&line) {
            println!("Received metrics from {}: {:?}", addr, metrics);
            
            // Store the metrics - using tokio's Mutex.lock().await instead
            let mut storage = metrics_storage.lock().await;
            storage
                .entry(metrics.hostname.clone())
                .or_insert_with(Vec::new)
                .push(metrics.clone());
            
            // Send acknowledgment
            writer.write_all(b"ACK\n").await?;
        } else {
            eprintln!("Invalid metrics format from {}: {}", addr, line);
            writer.write_all(b"ERROR: Invalid format\n").await?;
        }
    }

    Ok(())
}
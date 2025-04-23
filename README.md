# 🥑 Guacamole: Rust Monitoring System

<p align="center">
  <img src="https://www.rust-lang.org/logos/rust-logo-512x512.png" alt="Rust Logo" width="150" />
</p>

<p align="center">
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/Rust-1.70+-orange.svg" alt="Rust Version">
  </a>
  <a href="https://github.com/tokio-rs/tokio">
    <img src="https://img.shields.io/badge/Tokio-1.28-blue.svg" alt="Tokio Version">
  </a>
  <a href="https://github.com/serde-rs">
    <img src="https://img.shields.io/badge/Serde-1.0-green.svg" alt="Serde Version">
  </a>
  <a href="https://github.com/hyperium/hyper">
    <img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT">
  </a>
</p>

A distributed system monitoring application built with Rust. This project serves as both a learning tool for Rust networking concepts

## 📊 Features

- **Real-time Metrics Collection**: Gather CPU, memory, and disk usage from client machines
- **Centralized Storage**: Store metrics in a central server for analysis
- **Asynchronous Architecture**: Built with Tokio for efficient async networking
- **Thread-safe Concurrent Design**: Handle multiple clients simultaneously
- **Extensible Platform**: Designed to be built upon for more advanced monitoring features

## 🛠️ Architecture

The system consists of two main components:

**Client**:
- Collects system metrics from the local machine
- Sends metrics to the central server periodically
- Handles server acknowledgments

**Server**:
- Listens for client connections
- Processes and stores incoming metrics
- Provides acknowledgment responses
- Maintains connection with multiple clients

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or newer)
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/monitoring-system.git
   cd monitoring-system
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

### Running the Server

```bash
cargo run --bin server
```

The server will start listening on `127.0.0.1:8080`.

### Running the Client

```bash
cargo run --bin client
```

The client will connect to the server at `127.0.0.1:8080` and begin sending system metrics.

## 📚 Project Structure

```
monitoring-system/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   └── metrics.rs
└── src/bin/
    ├── server.rs
    └── client.rs
```

- **metrics.rs**: Defines the `SystemMetrics` structure shared between client and server
- **server.rs**: Implements the metrics collection server
- **client.rs**: Implements the metrics collecting client

## 🧪 Technical Details

### Dependencies

- **tokio**: Asynchronous runtime for Rust
- **serde**: Serialization/deserialization framework
- **systemstat**: System metrics collection library
- **anyhow**: Error handling
- **gethostname**: Hostname retrieval

### Data Flow

1. Client collects system metrics (CPU, memory, disk)
2. Client serializes metrics to JSON
3. Client sends JSON to server over TCP
4. Server receives and deserializes the data
5. Server stores metrics in memory (or database in advanced versions)
6. Server sends acknowledgment back to client
7. Client waits for next collection interval

## 🔜 Future Improvements

- [ ] Database integration for persistent storage
- [ ] Web dashboard for metrics visualization
- [ ] Alerting mechanism based on metric thresholds
- [ ] Authentication and encryption
- [ ] Service discovery for distributed setups
- [ ] Metrics aggregation and statistical analysis
- [ ] Container and Kubernetes monitoring capabilities
- [ ] Custom plugin system for additional metrics

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 🙏 Acknowledgments

- [Rust](https://www.rust-lang.org/) programming language
- [Tokio](https://tokio.rs/) async runtime

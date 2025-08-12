# PSYPSYPSY Backend

A Rust backend service built with Actix-web framework.

## Features

- ⚡ Fast and lightweight Actix-web server
- 🌐 CORS enabled for frontend integration
- 📝 JSON API responses
- 🔍 Health check endpoint
- 📋 Request logging middleware

## Prerequisites

- Rust (latest stable version)
- Cargo package manager

## Getting Started

### Installation

1. Install dependencies:
```bash
cargo build
```

2. Run the development server:
```bash
cargo run
```

The server will start at `http://localhost:8080`

### Available Endpoints

- `GET /` - Welcome message
- `GET /health` - Health check
- `GET /api/hello` - API hello endpoint

## Development

### Build for production:
```bash
cargo build --release
```

### Run tests:
```bash
cargo test
```

### Format code:
```bash
cargo fmt
```

### Lint code:
```bash
cargo clippy
```

## Project Structure

```
backend/
├── src/
│   └── main.rs          # Main server file
├── Cargo.toml           # Dependencies and project config
├── Cargo.lock           # Dependency lock file
└── README.md            # This file
```

## Configuration

The server runs on `127.0.0.1:8080` by default. You can modify the bind address in `src/main.rs`.

## Dependencies

- `actix-web` - Web framework
- `actix-cors` - CORS middleware
- `tokio` - Async runtime
- `serde` - Serialization framework
- `serde_json` - JSON serialization
- `env_logger` - Logging
- `log` - Logging facade

# macOS Log Manager

A secure and efficient tool for managing private logging on macOS systems.

## Features

- Enable private logging for specified subsystems
- Secure profile management
- Root privilege verification
- Proper error handling
- Async support

## Installation

```bash
cargo install macos-log-manager
```

## Usage

```rust
use macos_log_manager::LogManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = LogManager::new();
    manager.enable_private_logging(vec!["com.apple.XProtect".to_string()]).await?;
    Ok(())
}
```
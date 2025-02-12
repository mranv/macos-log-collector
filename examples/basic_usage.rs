use macos_log_manager::LogManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = LogManager::new();
    
    manager.enable_private_logging(vec!["com.apple.XProtect".to_string()]).await?;
    
    if manager.verify_config().await? {
        println!("Private logging enabled successfully");
    } else {
        println!("Failed to enable private logging");
    }

    Ok(())
}
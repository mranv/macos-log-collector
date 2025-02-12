use macos_log_manager::LogManager;
use tokio;

#[tokio::test]
async fn test_enable_private_logging() {
    let manager = LogManager::new();
    
    // Only run the actual test if we're running as root
    if unsafe { libc::geteuid() } == 0 {
        let result = manager
            .enable_private_logging(vec!["com.apple.XProtect".to_string()])
            .await;
        assert!(result.is_ok());
    } else {
        println!("Skipping test as it requires root privileges");
    }
}

#[tokio::test]
async fn test_verify_config() {
    let manager = LogManager::new();
    let result = manager.verify_config().await;
    assert!(result.is_ok());
}
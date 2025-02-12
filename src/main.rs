use std::path::PathBuf;
use clap::Parser;
use anyhow::{Result, Context};
use macos_log_manager::LogManager;

#[derive(Parser, Debug)]
#[command(name = "macos-log-manager")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Subsystems to enable private logging for (comma-separated)
    #[arg(short, long)]
    subsystems: Vec<String>,

    /// Output format (json or text)
    #[arg(short, long, default_value = "text")]
    format: String,

    /// Output file path
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Enable debug mode
    #[arg(short, long)]
    debug: bool,

    /// Verify configuration only
    #[arg(short, long)]
    verify: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize logger
    env_logger::init();

    // Create log manager instance
    let manager = LogManager::new();

    if args.verify {
        match manager.verify_config().await {
            Ok(true) => {
                println!("Private logging is properly configured");
                Ok(())
            }
            Ok(false) => {
                println!("Private logging is not properly configured");
                Ok(())
            }
            Err(e) => Err(anyhow::anyhow!("Failed to verify configuration: {}", e))
        }
    } else {
        // Enable private logging for specified subsystems
        manager.enable_private_logging(args.subsystems).await
            .context("Failed to enable private logging")?;

        println!("Private logging enabled successfully");

        // Verify the configuration if debug mode is enabled
        if args.debug {
            if manager.verify_config().await? {
                println!("Configuration verified successfully");
            } else {
                println!("Warning: Configuration verification failed");
            }
        }

        Ok(())
    }
}
use crate::{
    config::{LoggingProfile, SubsystemConfig},
    error::LoggingError,
    utils::{permissions, system},
};
use log::{info, warn};
use std::time::Duration;
use tokio::time::sleep;

pub struct LogManager {
    profile_path: String,
}

impl LogManager {
    pub fn new() -> Self {
        Self {
            profile_path: String::from("/Library/Preferences/Logging"),
        }
    }

    pub async fn enable_private_logging(&self, subsystems: Vec<String>) -> Result<(), LoggingError> {
        // Verify root privileges
        permissions::verify_root()?;
        info!("Root privileges verified");

        // Create configurations
        let configs: Vec<SubsystemConfig> = subsystems
            .iter()
            .map(|subsystem| {
                info!("Creating config for subsystem: {}", subsystem);
                SubsystemConfig::new(subsystem.clone())
            })
            .collect();

        let profile = LoggingProfile::new(configs);
        
        // Apply configuration
        self.apply_config(&profile).await?;
        info!("Configuration applied successfully");

        // Restart logging service
        system::restart_logging_service().await?;
        info!("Logging service restarted");

        // Wait for the service to fully restart
        sleep(Duration::from_secs(2)).await;

        // Verify immediately after enabling
        if !self.verify_config().await? {
            warn!("Initial configuration verification failed, attempting to fix...");
            // Try to apply settings directly
            system::apply_system_config(&self.profile_path).await?;
            system::set_logging_parameters().await?;
            
            // Check one more time
            if !self.verify_config().await? {
                warn!("Configuration verification still failed after retry");
            }
        }

        Ok(())
    }

    async fn apply_config(&self, profile: &LoggingProfile) -> Result<(), LoggingError> {
        // Write profile
        system::write_profile(&self.profile_path, profile).await?;
        info!("Profile written to disk");

        // Apply system configuration
        system::apply_system_config(&self.profile_path).await?;
        info!("System configuration applied");

        // Set additional logging parameters
        system::set_logging_parameters().await?;
        info!("Additional logging parameters set");

        Ok(())
    }

    pub async fn verify_config(&self) -> Result<bool, LoggingError> {
        // First check system logging configuration
        let sys_check = system::verify_logging_config().await?;
        if !sys_check {
            info!("System logging configuration check failed");
            return Ok(false);
        }
    
        // Then check XProtect specific configuration
        let xprotect_check = system::verify_subsystem_config().await?;
        if !xprotect_check {
            info!("XProtect configuration check failed");
            return Ok(false);
        }
    
        // Finally, verify we can access private logs
        let private_check = system::verify_private_logs().await?;
        if !private_check {
            info!("Private logs accessibility check failed");
            return Ok(false);
        }
    
        Ok(true)
    }
}
use std::process::Command;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use log::info;
use crate::error::LoggingError;
use crate::config::LoggingProfile;
use std::io::Cursor;

pub async fn write_profile(path: &str, profile: &LoggingProfile) -> Result<(), LoggingError> {
    let mut buf = Vec::new();
    plist::to_writer_xml(Cursor::new(&mut buf), &profile)
        .map_err(|e| LoggingError::ProfileError(e.to_string()))?;
    
    let plist_data = String::from_utf8(buf)
        .map_err(|e| LoggingError::ProfileError(e.to_string()))?;

    let file_path = format!("{}/private_logging.plist", path);
    info!("Writing profile to {}", file_path);
    
    let mut file = File::create(&file_path).await?;
    file.write_all(plist_data.as_bytes()).await?;
    
    Command::new("chmod")
        .args(&["644", &file_path])
        .output()
        .map_err(|e| LoggingError::CommandError(e.to_string()))?;

    Ok(())
}

pub async fn apply_system_config(_path: &str) -> Result<(), LoggingError> {
    info!("Applying system configuration...");

    // Use correct mode syntax for log command
    let result = Command::new("log")
        .args(&["config", "--mode", "level:debug"])
        .output()
        .map_err(|e| LoggingError::CommandError(e.to_string()))?;
    info!("Debug level setting result: {:?}", result);

    // Enable private data collection
    let result = Command::new("defaults")
        .args(&[
            "write",
            "/Library/Preferences/Logging/com.apple.system.logging",
            "System.Private-Data",
            "-bool",
            "true"
        ])
        .output()
        .map_err(|e| LoggingError::CommandError(e.to_string()))?;
    info!("System private data setting result: {:?}", result);

    Ok(())
}

pub async fn set_logging_parameters() -> Result<(), LoggingError> {
    info!("Setting additional logging parameters...");

    // Configure XProtect
    let commands = [
        // Enable basic logging
        ("defaults", &[
            "write",
            "/Library/Preferences/Logging/com.apple.XProtect",
            "Enable-Logging",
            "-bool",
            "true"
        ]),
        // Set debug level
        ("defaults", &[
            "write",
            "/Library/Preferences/Logging/com.apple.XProtect",
            "Level",
            "-string",
            "Debug"
        ]),
        // Enable private data
        ("defaults", &[
            "write",
            "/Library/Preferences/Logging/com.apple.XProtect",
            "Private",
            "-bool",
            "true"
        ]),
        // Set category settings
        ("defaults", &[
            "write",
            "/Library/Preferences/Logging/com.apple.XProtect",
            "Category-Default-Enabled",
            "-bool",
            "true"
        ])
    ];

    for (cmd, args) in commands.iter() {
        let result = Command::new(cmd).args(*args).output()
            .map_err(|e| LoggingError::CommandError(e.to_string()))?;
        info!("Command {} {:?} result: {:?}", cmd, args, result);
    }

    Ok(())
}

pub async fn verify_subsystem_config() -> Result<bool, LoggingError> {
    info!("Verifying subsystem configuration...");

    // Read XProtect configuration
    let xprotect_output = Command::new("defaults")
        .args(&["read", "/Library/Preferences/Logging/com.apple.XProtect"])
        .output()
        .map_err(|e| LoggingError::CommandError(e.to_string()))?;

    let xprotect_config = String::from_utf8_lossy(&xprotect_output.stdout);
    info!("XProtect configuration: {}", xprotect_config);

    // Check for the presence of required keys and values
    let required_settings = [
        "\"Enable-Logging\" = 1",
        "\"Category-Default-Enabled\" = 1",
        "Level = Debug",
        "Private = 1"
    ];

    // Log each setting check
    let mut settings_status = vec![];
    for setting in required_settings.iter() {
        let normalized_setting = setting.replace(" ", "");
        let normalized_config = xprotect_config
            .replace(" ", "")
            .replace("\n", "")
            .replace("\t", "");
        
        let is_present = normalized_config.contains(&normalized_setting);
        info!("Checking setting '{}': {}", setting, is_present);
        
        if !is_present {
            info!("Missing required setting: {}", setting);
            settings_status.push(false);
        } else {
            settings_status.push(true);
        }
    }

    // All settings must be present
    let all_settings_present = settings_status.iter().all(|&x| x);
    info!("XProtect settings verification result: {}", all_settings_present);

    // Verify system status
    let sys_output = Command::new("log")
        .args(&["config", "--status"])
        .output()
        .map_err(|e| LoggingError::CommandError(e.to_string()))?;

    let sys_status = String::from_utf8_lossy(&sys_output.stdout);
    info!("System status: {}", sys_status);

    let has_debug = sys_status.contains("DEBUG");
    let has_private = sys_status.contains("PRIVATE_DATA");
    
    info!("System status check - Debug: {}, Private: {}", has_debug, has_private);

    // Both configuration and system status must be correct
    Ok(all_settings_present && has_debug && has_private)
}

pub async fn verify_private_logs() -> Result<bool, LoggingError> {
    info!("Verifying private logs are accessible...");

    // Try to read XProtect logs including private data
    let output = Command::new("log")
        .args(&["show", "--predicate", "subsystem == 'com.apple.XProtect'", "--style", "json", "--debug", "--last", "1m"])
        .output()
        .map_err(|e| LoggingError::CommandError(e.to_string()))?;

    let log_content = String::from_utf8_lossy(&output.stdout);
    info!("Log sample: {}", log_content);

    // Check if we're seeing private data (not seeing <private> tags)
    let has_private_data = !log_content.contains("<private>") && log_content.len() > 0;
    info!("Private data check: {}", has_private_data);

    Ok(has_private_data)
}

pub async fn restart_logging_service() -> Result<(), LoggingError> {
    info!("Restarting logging service...");
    
    // Reset logging configuration
    Command::new("log")
        .args(&["config", "--mode", "level:default"])
        .output()
        .map_err(|e| LoggingError::CommandError(e.to_string()))?;

    // Stop logging service
    Command::new("killall")
        .arg("logd")
        .output()
        .map_err(|e| LoggingError::CommandError(e.to_string()))?;

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Re-enable debug logging
    Command::new("log")
        .args(&["config", "--mode", "level:debug"])
        .output()
        .map_err(|e| LoggingError::CommandError(e.to_string()))?;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    Ok(())
}

pub async fn verify_logging_config() -> Result<bool, LoggingError> {
    info!("Verifying logging configuration...");
    
    // Check system-wide configuration
    let output = Command::new("log")
        .args(&["config", "--status"])
        .output()
        .map_err(|e| LoggingError::CommandError(e.to_string()))?;

    let status = String::from_utf8_lossy(&output.stdout);
    info!("System logging status: {}", status);

    // Parse the status output more carefully
    let has_debug = status.contains("DEBUG");
    let has_private = status.contains("PRIVATE_DATA");
    let is_live = status.contains("STREAM_LIVE"); // This is normal and expected

    info!("Status check - Debug: {}, Private: {}, Live: {}", 
          has_debug, has_private, is_live);

    // Consider configuration valid if we have both DEBUG and PRIVATE_DATA
    Ok(has_debug && has_private)
}

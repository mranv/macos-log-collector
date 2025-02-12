use serde::{Serialize, Deserialize};
use super::subsystem::SubsystemConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggingProfile {
    pub payload_content: Vec<SubsystemConfig>,
    pub payload_identifier: String,
    pub payload_uuid: String,
    pub payload_type: String,
    pub payload_version: i32,
}

impl LoggingProfile {
    pub fn new(subsystems: Vec<SubsystemConfig>) -> Self {
        Self {
            payload_content: subsystems,
            payload_identifier: "com.security.logging".to_string(),
            payload_uuid: uuid::Uuid::new_v4().to_string(),
            payload_type: "Configuration".to_string(),
            payload_version: 1,
        }
    }
}
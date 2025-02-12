use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubsystemConfig {
    pub subsystem: String,
    pub level: String,
    pub private_data: bool,
    pub categories: Vec<String>,
}

impl SubsystemConfig {
    pub fn new(subsystem: String) -> Self {
        Self {
            subsystem,
            level: "Debug".to_string(),
            private_data: true,
            categories: vec!["behavior".to_string(), "scanner".to_string()],
        }
    }
}
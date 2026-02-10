use serde::{
    Deserialize,
    Serialize
};

use std::fs;

#[derive(Serialize, Deserialize)]
pub struct SandboxConfig {
    pub timeout_seconds: u64,
    pub max_memory_mb: u64,
    pub max_code_size_kb: u64,
    pub allow_network: bool,
    pub allow_file_io: bool,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        SandboxConfig {
            timeout_seconds: 5,
            max_memory_mb: 128,
            max_code_size_kb: 10,
            allow_network: false,
            allow_file_io: false,
        }
    }
}

impl SandboxConfig {
    /// Load yaml config
    pub fn from_yaml_file(path_to_yaml_file: String) -> Result<Self, String> {
        let content = fs::read_to_string(&path_to_yaml_file).map_err(|e| format!("Can not read file {}: {}", path_to_yaml_file, e))?;
        serde_yaml::from_str(&content).map_err(|e| format!("YAML pase error: {}", e))
    }
}

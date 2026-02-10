mod config;
mod func;

use config::SandboxConfig;

pub struct Sandbox {
    timeout_seconds: u64,
    max_code_size_kb: u64,
}

impl Sandbox {
    /// Create new instance of Sandbox
    fn new(config: SandboxConfig) -> Self {
        Sandbox {
            timeout_seconds: config.timeout_seconds,
            max_code_size_kb: config.max_code_size_kb,
        }
    }

    /// Getters
    pub fn timeout_seconds(&self) -> u64 {
        self.timeout_seconds
    }

    pub fn max_code_size_kb(&self) -> u64 {
        self.max_code_size_kb
    }

    /// Create Sandbox from yaml
    pub fn from_yaml_file(path: String) -> Result<Self, String> {
        let config = SandboxConfig::from_yaml_file(path)?;
        Ok(Sandbox::new(config))
    }

    /// Create default Sandbox
    pub fn default() -> Self {
        Sandbox::new(SandboxConfig::default())
    }
}

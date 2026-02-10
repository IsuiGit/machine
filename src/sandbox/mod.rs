mod config;
mod func;

use config::SandboxConfig;

pub struct Sandbox {
    timeout_seconds: u64,
    max_memory_mb: u64,
    max_code_size_kb: u64,
    allow_network: bool,
    allow_file_io: bool,
}

impl Sandbox {
    /// Create new instance of Sandbox
    fn new(config: SandboxConfig) -> Self {
        Sandbox {
            timeout_seconds: config.timeout_seconds,
            max_memory_mb: config.max_memory_mb,
            max_code_size_kb: config.max_code_size_kb,
            allow_network: config.allow_network,
            allow_file_io: config.allow_file_io,
        }
    }

    /// Getters
    pub fn timeout_seconds(&self) -> u64 {
        self.timeout_seconds
    }

    pub fn max_memory_mb(&self) -> u64 {
        self.max_memory_mb
    }

    pub fn max_code_size_kb(&self) -> u64 {
        self.max_code_size_kb
    }

    pub fn allow_network(&self) -> bool {
        self.allow_network
    }

    pub fn allow_file_io(&self) -> bool {
        self.allow_file_io
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

pub mod func;
mod info;

pub use info::*;

use std::path::PathBuf;
use func::remove_environment;

use crate::logger::logger;

pub struct PythonInfo {
    path: PathBuf,
    version: String,
}

impl PythonInfo {
    /// Getters
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn version(&self) -> String {
        self.version.clone()
    }
}

pub struct EnvironmentInfo {
    path: PathBuf,
    executable: PathBuf,
    activate: PathBuf,
    version: String,
}

impl EnvironmentInfo {
    /// Getters
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn executable(&self) -> PathBuf {
        self.executable.clone()
    }

    pub fn activate(&self) -> PathBuf {
        self.activate.clone()
    }

    pub fn version(&self) -> String {
        self.version.clone()
    }
}

impl Drop for EnvironmentInfo {
    fn drop(&mut self) {
        if let Err(e) = remove_environment(&self.path) {
            logger().error(format!("Warning: failed to cleanup venv at {:?}: {}", self.path, e));
        }
    }
}

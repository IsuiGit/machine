use crate::cli::Command;
use crate::sandbox::Sandbox;
use crate::python::{
    get_python_info,
    get_environment_info,
};
use crate::logger::logger;

use std::fmt::Display;

pub struct App;

impl App {
    pub fn exec(self, command: Command) -> Result<(), String> {
        match command {
            Command::CheckPython => {
                handle_result(get_python_info(), |info| {
                    logger().info("--- Python info from PATH ---".to_string());
                    logger().info(format!("Path: {}", info.path().display()));
                    logger().info(format!("Version: {}", info.version()));
                })?
            }
            Command::CheckEnvironment => {
                handle_result(get_environment_info(), |info| {
                    logger().info("--- Python info from env ---".to_string());
                    logger().info(format!("Path to environment: {}", info.path().display()));
                    logger().info(format!("Path to executable: {}", info.executable().display()));
                    logger().info(format!("Path to activate.bat: {}", info.activate().display()));
                    logger().info(format!("Python environment version: {}", info.version()));
                })?
            }
            Command::CheckSandboxCapabilities { path_to_yaml_file: path } => {
                let sandbox = if path.is_empty() { Sandbox::default() } else { Sandbox::from_yaml_file(path.clone())? };
                logger().info(format!("--- Sandbox capabilities ({}) ---", if path.is_empty() { "default" } else { &path }));
                logger().info(format!("Timeout (seconds): {}", sandbox.timeout_seconds()));
                logger().info(format!("Max code size (KB): {}", sandbox.max_code_size_kb()));
            }
            Command::Run { path_to_script: path, path_to_yaml_file: yaml_path, host: listener_host, port: listener_port} => {
                let sandbox = if yaml_path.is_empty() { Sandbox::default() } else { Sandbox::from_yaml_file(yaml_path.clone())? };
                sandbox.run(path, listener_host, listener_port).map(|_| logger().info("Script execution successfully complete".to_string())).map_err(|e| e)?
            }
        }
        Ok(())
    }
}

// TO DO: learn how does it works
fn handle_result<T, E: Display>(result: Result<T, E>, on_success: impl FnOnce(T)) -> Result<(), String> {
    match result {
        Ok(value) => Ok(on_success(value)),
        Err(e) => return Err(format!("Error: {}", e)),
    }
}

// Use instancies:
// - cli::Command for identify incoming comma
// - sandbox instance
// - python info funcs
// - logger instance
use crate::cli::Command;
use crate::sandbox::Sandbox;
use crate::python::{
    get_python_info,
    get_environment_info,
};
use crate::logger::logger;

// Use display for transform Path to Strings
use std::fmt::Display;

// Create static instance
pub struct App;

// Impl funcs for app
impl App {
    // Main executor function
    pub fn exec(self, command: Command) -> Result<(), String> {
        // Match incoming comma type
        match command {
            // CheckPython comma search for Python executable on device
            Command::CheckPython => {
                // Handle result:
                // - if function successefuly ended logging Python info
                // - if function ended with exception raise error on main level
                handle_result(get_python_info(), |info| {
                    logger().info("--- Python info from PATH ---".to_string());
                    logger().info(format!("Path: {}", info.path().display()));
                    logger().info(format!("Version: {}", info.version()));
                })?
            }
            // CheckEnvironment comma get Python and create env
            Command::CheckEnvironment => {
                // Handle result:
                // - if function successefuly ended logging environment info
                // - if function ended with exception raise error on main level
                handle_result(get_environment_info(), |info| {
                    logger().info("--- Python info from env ---".to_string());
                    logger().info(format!("Path to environment: {}", info.path().display()));
                    logger().info(format!("Path to executable: {}", info.executable().display()));
                    logger().info(format!("Path to activate.bat: {}", info.activate().display()));
                    logger().info(format!("Python environment version: {}", info.version()));
                })?
            }
            // CheckSandboxCapabilities comma get Sandbox info
            Command::CheckSandboxCapabilities { path_to_yaml_file: path } => {
                // Create sandbox from file or default:
                // - if path to sandbox yaml file was sended create Sandbox from file
                // - if path to sandbox yaml file was not sended create default Sandbox
                // - if creat sandbox func ended with exception raise error on main level
                let sandbox = if path.is_empty() { Sandbox::default() } else { Sandbox::from_yaml_file(path.clone())? };
                // Logging sanbox info
                logger().info(format!("--- Sandbox capabilities ({}) ---", if path.is_empty() { "default" } else { &path }));
                logger().info(format!("Timeout (seconds): {}", sandbox.timeout_seconds()));
                logger().info(format!("Max code size (KB): {}", sandbox.max_code_size_kb()));
            }
            // Run comma start proccess of execution Python script
            Command::Run { path_to_script: path, path_to_yaml_file: yaml_path, host: listener_host, port: listener_port} => {
                // Create sandbox from file or default:
                // - if path to sandbox yaml file was sended create Sandbox from file
                // - if path to sandbox yaml file was not sended create default Sandbox
                // - if creat sandbox func ended with exception raise error on main level
                let sandbox = if yaml_path.is_empty() { Sandbox::default() } else { Sandbox::from_yaml_file(yaml_path.clone())? };
                // Start process of execution script
                // - if function ended successefuly return result of execution
                // - if function ended with exception raise error on main level
                sandbox.run(path, listener_host, listener_port).map(|_| logger().info("Script execution successfully complete".to_string())).map_err(|e| e)?
            }
        }
        // If comma ended successefuly
        Ok(())
    }
}

// Abstract handler
fn handle_result<T, E: Display>(result: Result<T, E>, on_success: impl FnOnce(T)) -> Result<(), String> {
    match result {
        Ok(value) => Ok(on_success(value)),
        Err(e) => return Err(format!("Error ({})", e)),
    }
}

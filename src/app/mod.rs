use crate::cli::Command;
use crate::sandbox::Sandbox;
use crate::python::{
    get_python_info,
    get_environment_info,
};

use std::fmt::Display;

pub struct App;

impl App {
    pub fn exec(self, command: Command) -> Result<(), String> {
        match command {
            Command::CheckPython => {
                handle_result(get_python_info(), |info| {
                    println!("--- Python info from PATH ---");
                    println!("Path: {}", info.path().display());
                    println!("Version: {}", info.version());
                })
            }
            Command::CheckEnvironment => {
                handle_result(get_environment_info(), |info| {
                    println!("--- Python info from env ---");
                    println!("Path to environment: {}", info.path().display());
                    println!("Path to executable: {}", info.executable().display());
                    println!("Path to activate.bat: {}", info.activate().display());
                    println!("Python environment version: {}", info.version());
                })
            }
            Command::CheckSandboxCapabilities { path_to_yaml_file: path } => {
                let sandbox = if path.is_empty() { Sandbox::default() } else { Sandbox::from_yaml_file(path.clone())? };
                println!("--- Sandbox capabilities ({}) ---",
                    if path.is_empty() { "default" } else { &path });
                println!("Timeout (seconds): {}", sandbox.timeout_seconds());
                println!("Max memory (MB): {}", sandbox.max_memory_mb());
                println!("Max code size (KB): {}", sandbox.max_code_size_kb());
                println!("Allow network: {}", sandbox.allow_network());
                println!("Allow file I/O: {}", sandbox.allow_file_io());
            }
            Command::Run { path_to_script: path, path_to_yaml_file: yaml_path } => {
                let sandbox = if yaml_path.is_empty() { Sandbox::default() } else { Sandbox::from_yaml_file(yaml_path.clone())? };
                sandbox.run(path).map(|_| println!("Script execution successfully complete")).map_err(|e| e)?
            }
        }
        Ok(())
    }
}

// TO DO: learn how does it works
fn handle_result<T, E: Display>(result: Result<T, E>, on_success: impl FnOnce(T)) {
    match result {
        Ok(value) => on_success(value),
        Err(e) => eprintln!("{}", e),
    }
}

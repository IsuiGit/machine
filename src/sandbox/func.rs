use super::Sandbox;
use super::utils::copy_file;

use crate::python::func::{
    make_environment,
    get_environment,
    remove_environment,
};

use std::{
    path::Path,
    process::Command,
};

impl Sandbox{
    pub fn run(&self, script_path: String) -> Result<(), String>{
        let venv_path = make_environment()?;
        let venv = get_environment(venv_path.clone())?;
        let file = copy_file(&Path::new(&script_path), &venv.path())?;
        let mut command = Command::new(&venv.executable());
        command.current_dir(&venv.path());
        command.arg(&file);
        let status = command.status().map_err(|e| format!("Failed to execute: {}", e))?;
        if status.success() {
            remove_environment(venv_path.clone())?;
            Ok(())
        } else {
            remove_environment(venv_path.clone())?;
            Err(format!("Script failed with exit code: {}", status))
        }
    }
}

use super::Sandbox;
use super::utils::{copy_file, get_last_dir};

use crate::python::func::{
    make_environment,
    get_environment,
    remove_environment,
};

use std::{
    env,
    fs::File,
    process::{Stdio, Command},
    path::{Path, PathBuf},
};

impl Sandbox{
    pub fn run(&self, script_path: String) -> Result<(), String>{
        let venv_path = make_environment()?;
        let venv = get_environment(venv_path.clone())?;
        let file = copy_file(&Path::new(&script_path), &venv.path())?;
        let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("C:\\"));
        let output = get_last_dir(&mut venv.path())
            .map(|s| format!("{}.machine", s))
            .unwrap_or_default();
        let output_io = File::create(cwd.join(output)).map_err(|e| e.to_string())?;
        let output_err = output_io.try_clone().map_err(|e| e.to_string())?;
        let mut command = Command::new(&venv.executable())
            .current_dir(&venv.path())
            .arg(&file)
            .stdout(Stdio::from(output_io))
            .stderr(Stdio::from(output_err))
            .status()
            .map_err(|e| e.to_string())?;
        if command.success() {
            remove_environment(venv.path())?;
            Ok(())
        } else {
            remove_environment(venv.path())?;
            Err(format!("Failed to execute: {}", command))
        }
    }
}

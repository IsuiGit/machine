use super::Sandbox;
use crate::utils::*;

use crate::python::func::{
    make_environment,
    get_environment,
};

use crossbeam::channel::{
    self,
    tick,
    select,
};

use std::{
    fs::File,
    io::Write,
    process::Output,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Duration,
};

impl Sandbox{
    pub fn run(&self, script_path: String) -> Result<(), String>{
        let venv_path = make_environment()?;
        let venv = get_environment(&venv_path)?;
        let file = copy_file(&Path::new(&script_path), &venv.path())?;
        let file_size = get_file_size_kb(&Path::new(&file))?;
        if file_size > self.max_code_size_kb {
            return Err("Your file size more than max file size (in kb)".to_string())
        }
        let cwd = get_cwd();
        let output = cwd.join(get_last(&mut venv.path())
            .map(|s| format!("{}.machine", s))
            .unwrap_or_default()
        );
        let execution_result = self.execute_with_tiemout(venv.executable(), venv.path(), file)?;
        let stdout = &execution_result.stdout;
        let stderr = &execution_result.stderr;
        if !stderr.is_empty() {
            return Err(String::from_utf8_lossy(stderr).to_string());
        }
        let mut file = File::create(output).map_err(|e| format!("{}", e))?;
        file.write_all(stdout).map_err(|e| format!("{}", e))?;
        Ok(())
    }

    // Learn it earlier
    fn execute_with_tiemout(&self, exec: PathBuf, dir: PathBuf, arg: String) -> Result<Output, String> {
        // Main channels
        let (tx, rx) = channel::unbounded();
        // Bind comma
        let child = spawn_child_process(&exec, &dir, &arg)?;
        // Wrap in Arc<Mutex> for shared ownership
        let child_arc = Arc::new(Mutex::new(Some(child)));
        let child_for_thread = Arc::clone(&child_arc);
        // Spawn comma
        let binding_handle = spawn_execution_thread(child_for_thread, tx);
        // Set timer
        let timer = tick(Duration::from_secs(self.timeout_seconds));
        // Select Ok or Kill
        select! {
            recv(rx) -> msg => {
                handle_command_completion(msg, binding_handle)
            },
            recv(timer) -> _ => {
                handle_timeout(self.timeout_seconds, binding_handle, child_arc)
            }
        }
    }
}

use super::Sandbox;
use crate::utils::*;

use crate::python::func::{
    make_environment,
    get_environment,
};

use crossbeam::channel::{
    self,
    select,
    tick,
};

use std::{
    fs::File,
    io::Write,
    process::{Command, Output},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread::spawn,
    time::Duration,
    process::Stdio,
    io::{ErrorKind, Error}
};

impl Sandbox{
    pub fn run(&self, script_path: String) -> Result<(), String>{
        let venv_path = make_environment()?;
        let venv = get_environment(&venv_path)?;
        let file = copy_file(&Path::new(&script_path), &venv.path())?;
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
        let child = Command::new(&exec)
            .current_dir(&dir)
            .arg(&arg)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start: {}", e))?;
        // Wrap in Arc<Mutex> for shared ownership
        let child_arc = Arc::new(Mutex::new(Some(child)));
        let child_for_thread = Arc::clone(&child_arc);
        // Spawn comma
        let binding_handle = spawn(move || {
            // Take ownership of the Child from the Mutex
            let child_opt = child_for_thread.lock().unwrap().take();
            if let Some(child) = child_opt {
                let result = child.wait_with_output();
                let _ = tx.send(result);
            } else {
                let _ = tx.send(
                    ErrError::new(ErrorKind::Other,"Child already taken")));
            }
        });
        // Set timer
        let timer = tick(Duration::from_secs(self.timeout_seconds));
        // Select Ok or Kill
        select!{
            recv(rx) -> msg => {
                binding_handle.join().ok();
                match msg {
                    Ok(Ok(output)) => Ok(output),
                    Ok(Err(e)) => Err(format!("Command failed: {}", e)),
                    Err(_) => Err("Channel error".into()),
                }
            },
            recv(timer) -> _ => {
                // Try to take the child to kill it
                if let Some(mut child) = child_arc.lock().unwrap().take() {
                    child.kill().ok();
                    child.wait().ok();
                } else {
                    // Child was already taken by the thread
                    // The thread might still be waiting, so we can't kill directly
                }
                binding_handle.join().ok();
                Err(format!("Timeout after {} seconds", self.timeout_seconds))
            }
        }
    }
}

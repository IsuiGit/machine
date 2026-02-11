use super::Sandbox;
use crate::utils::*;

use crate::python::func::{
    make_environment,
    get_environment,
};

use std::{
    fs::File,
    io::Write,
    process::{Command, Output, Stdio},
    path::Path,
    sync::mpsc::{channel, RecvTimeoutError},
    time::Duration,
    thread,
};

impl Sandbox{
    // Main executable
    pub fn run(&self, script_path: String) -> Result<(), String>{
        let venv_path = make_environment()?;
        let venv = get_environment(&venv_path)?;
        let file = copy_file(&Path::new(&script_path), &venv.path())?;
        println!("Copied script file into: {}", file.display());
        let file_size = get_file_size_kb(file.as_ref())?;
        if file_size > self.max_code_size_kb {
            return Err("Your file size more than max file size (in kb)".to_string())
        }
        let cwd = get_cwd();
        let filename = match get_last(venv.path().as_ref()) {
            Some(name) => format!("{}.machine", name),
            None => String::from("unregistered.machine"),
        };
        let output = cwd.join(filename);
        let script_path_as_str = match file.as_os_str().to_str() {
            Some(s) => s,
            None => return Err("Can't transform path from script file to string".to_string()),
        };
        let args: &[&str] = &[script_path_as_str];
        println!("Start script execution");
        let execution_result = self.execute_with_timeout(
            venv.executable().as_ref(),
            venv.path().as_ref(),
            &args,
            Duration::from_secs(self.timeout_seconds)
        )?;
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
    fn execute_with_timeout(&self, exec: &Path, dir: &Path, args: &[&str], timeout: Duration) -> Result<Output, String> {
        // Start child process
        let child = Command::new(exec)
            .current_dir(dir)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn process: {}", e))?;
        // Create sender/receiver pair
        let (sender, receiver) = channel();
        // Get child process id
        let child_id = child.id();
        // Run child in thread
        thread::spawn(move || {
            match child.wait_with_output() {
                Ok(output) => {
                    let _ = sender.send(Ok(output));
                }
                Err(e) => {
                    let _ = sender.send(Err(format!("Process error: {}", e)));
                }
            }
        });
        // Wait thread result
        match receiver.recv_timeout(timeout) {
            Ok(result) => result,
            Err(RecvTimeoutError::Timeout) => {
                force_kill_process(child_id)?;
                Err(format!("Timeout after {:?}", timeout))
            }
            Err(RecvTimeoutError::Disconnected) => {
                force_kill_process(child_id)?;
                Err("Thread died unexpectedly".to_string())
            }
        }
    }
}

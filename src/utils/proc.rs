use std::{
    path::PathBuf,
    process::{Output, Child, Command, Stdio},
    sync::{Arc, Mutex},
    thread::{spawn, JoinHandle},
    io::{Error, ErrorKind},
};

use crossbeam::channel::{
    Sender,
};

// Spawn child process
pub fn spawn_child_process(exec: &PathBuf, dir: &PathBuf, arg: &String) -> Result<Child, String> {
    Command::new(exec)
        .current_dir(dir)
        .arg(arg)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start: {}", e))
}

// Kill proc
pub fn kill_child_process(mut child: Child) {
    child.kill().ok();
    child.wait().ok();
}

// Spawn child executor and handle
pub fn spawn_execution_thread(child_arc: Arc<Mutex<Option<Child>>>, tx: Sender<Result<Output, Error>>) -> JoinHandle<()> {
    spawn(move || {
        // Берем владение Child из Mutex
        let child_opt = child_arc.lock().unwrap().take();
        if let Some(child) = child_opt {
            let result = child.wait_with_output();
            let _ = tx.send(result);
        } else {
            let _ = tx.send(
                Err(Error::new(ErrorKind::Other, "Child already taken"))
            );
        }
    })
}

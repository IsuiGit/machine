use std::{
    process::{Output, Child},
    sync::{Arc, Mutex},
    thread::JoinHandle,
    io::Error,
};

use crossbeam::channel::{
    RecvError,
};

use super::proc::kill_child_process;

// Waited for handle compl
pub fn handle_command_completion(handle_message: Result<Result<Output, Error>, RecvError>, handle: JoinHandle<()>) -> Result<Output, String> {
    handle.join().ok();
    match handle_message {
        Ok(Ok(output)) => Ok(output),
        Ok(Err(e)) => Err(format!("Command failed: {}", e)),
        Err(_) => Err("Channel error".into()),
    }
}

// Handle timeout process
pub fn handle_timeout(time: u64, handle: JoinHandle<()>, child_arc: Arc<Mutex<Option<Child>>>) -> Result<Output, String> {
    if let Some(child) = child_arc.lock().unwrap().take() { kill_child_process(child); } else {}
    handle.join().ok();
    Err(format!("Timeout after {} seconds", time))
}

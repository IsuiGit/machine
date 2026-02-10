use std::process::Command;

pub fn force_kill_process(pid: u32) {
    let _ = Command::new("taskkill").args(&["/PID", &pid.to_string(), "/F", "/T"]).spawn().and_then(|mut c| c.wait());
}

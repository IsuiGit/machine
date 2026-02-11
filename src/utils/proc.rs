use std::process::Command;

pub fn force_kill_process(pid: u32) -> Result<(), String> {
    let status = Command::new("taskkill")
        .args(&["/PID", &pid.to_string(), "/F", "/T"])
        .status()
        .map_err(|e| format!("Failed to execute taskkill: {}", e))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("taskkill failed with status: {}", status))
    }
}

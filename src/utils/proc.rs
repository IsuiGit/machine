use std::{
    process::Command,
    time::Duration,
    thread::sleep,
};

pub fn force_kill_process(pid: u32) -> Result<(), String> {
    println!("Sleep for 1s");
    sleep(Duration::from_secs(1));
    let check = Command::new("tasklist")
        .args(&["/FI", &format!("PID eq {}", pid), "/NH"])
        .output()
        .map_err(|e| format!("Failed to execute tasklist: {}", e))?;
    let output = String::from_utf8_lossy(&check.stdout);
    if output.contains(&pid.to_string()) {
        let status = Command::new("taskkill")
            .args(&["/PID", &pid.to_string(), "/F", "/T"])
            .status()
            .map_err(|e| format!("Failed to execute taskkill: {}", e))?;
        if status.success() { return Ok(()) } else { return Err(format!("taskkill failed with status: {}", status)) }
    } else {
        println!("Process with pid {} was also ended", pid);
        Ok(())
    }
}

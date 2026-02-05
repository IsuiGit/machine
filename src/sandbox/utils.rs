use std::fs;
use std::path::{Path, PathBuf};

pub fn copy_file(source: &Path, destination: &Path) -> Result<String, String> {
    // Check source exists
    if !source.exists() {
        return Err(format!("File not found: {}", source.display()));
    }
    // Check source is file
    if !source.is_file() {
        return Err(format!("Not a file: {}", source.display()));
    }
    // Get file name
    let file_name = source.file_name().ok_or("Can't get file name")?.to_string_lossy();
    // Final destination
    let dst = destination.join(&*file_name);
    // Copy sript file into venv
    let _ = fs::copy(source, &dst).map_err(|e| format!("Error on copy script: {}", e));
    // return filename
    Ok(file_name.to_string())
}

pub fn get_last_dir(path: &mut PathBuf) -> Option<&str> {
    if let Some(s) = path.to_str(){
        if s.ends_with('/') || s.ends_with('\\') {
            path.pop();
        }
    }
    path.file_name()?.to_str()
}

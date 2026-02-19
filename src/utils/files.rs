use std::{
    path::{Path, PathBuf},
    fs,
    io::ErrorKind,
};

use crate::logger::logger;

// Abstaract checker of non-empty file
pub fn is_non_empty_file(path: &Path) -> Result<bool, String> {
    match fs::metadata(path) {
        Ok(metadata) => Ok(metadata.is_file() && metadata.len() > 0),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Err(format!("File '{}' not found on non-empty check", path.display())),
            ErrorKind::PermissionDenied => Err(format!("No access for '{}'", path.display())),
            _ => Err(format!("Error in file_size: {}", e)),
        }
    }
}

// Abstract iterator for file search
pub fn find_file_in_dir(dir: &Path, names: &[&str]) -> Result<Option<PathBuf>, String> {
    for name in names {
        let candidate = dir.join(name);
        match is_non_empty_file(&candidate) {
            Ok(true) => return Ok(Some(candidate)),
            Ok(false) => continue,
            Err(_) => continue,
        }
    }
    Ok(None)
}

// Get file size
pub fn get_file_size_kb(path: &Path) -> Result<u64, String> {
    match fs::metadata(path) {
        Ok(metadata) => Ok(metadata.len() / 1024),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Err(format!("File '{}' not found on filesize check", path.display())),
            ErrorKind::PermissionDenied => Err(format!("No access for '{}'", path.display())),
            _ => Err(format!("Error in file_size: {}", e)),
        }
    }
}

// Abstract file copy method
pub fn copy_file(source: &Path, destination: &Path) -> Result<PathBuf, String> {
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
    logger().info(format!("Destination filepath: {}", dst.display()));
    // Copy sript file into venv
    fs::copy(source, &dst).map_err(|e| format!("Error on copy script: {}", e))?;
    // return filename
    Ok(dst)
}

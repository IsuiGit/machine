use std::{
    fs,
    env,
    path::{Path, PathBuf},
};

// Abstaract checker of non-empty file
fn is_non_empty_file(path: &Path) -> bool {
    path.exists() && fs::metadata(path).map(|meta| meta.is_file() && meta.len() > 0).unwrap_or(false)
}

// Abstract iterator for file search
pub fn find_file_in_dir(dir: &Path, names: &[&str]) -> Option<PathBuf> {
    names.iter().map(|name| dir.join(name)).find(|candidate| is_non_empty_file(candidate))
}

// Abstract file copy method
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

// Abstract last dir/file in path
pub fn get_last(path: &mut PathBuf) -> Option<&str> {
    if let Some(s) = path.to_str(){
        if s.ends_with('/') || s.ends_with('\\') {
            path.pop();
        }
    }
    path.file_name()?.to_str()
}

pub fn get_cwd() -> PathBuf {
    env::current_dir().unwrap_or_else(|_| PathBuf::from("C:\\"))
}

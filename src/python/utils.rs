use std::{
    fs,
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

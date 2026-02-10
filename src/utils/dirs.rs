use std::{
    path::{Path, PathBuf},
    env,
};

// Abstract last dir/file in path
pub fn get_last(path: &Path) -> Option<&str> {
    let components: Vec<_> = path.components().collect();
    components.last()
        .and_then(|comp| comp.as_os_str().to_str())
        .filter(|s| !s.is_empty())
}

// Getting current work directory
pub fn get_cwd() -> PathBuf {
    env::current_dir().unwrap_or_else(|_| PathBuf::from("C:\\"))
}

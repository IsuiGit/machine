use std::{
    path::PathBuf,
    env,
};

// Abstract last dir/file in path
pub fn get_last(path: &mut PathBuf) -> Option<&str> {
    if let Some(s) = path.to_str(){
        if s.ends_with('/') || s.ends_with('\\') {
            path.pop();
        }
    }
    path.file_name()?.to_str()
}

// Getting current work directory
pub fn get_cwd() -> PathBuf {
    env::current_dir().unwrap_or_else(|_| PathBuf::from("C:\\"))
}

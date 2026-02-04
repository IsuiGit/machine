use super::utils::*;
use super::{PythonInfo, EnvironmentInfo};

use std::{
    fs,
    env,
    path::PathBuf,
    process::Command,
};

use uuid::Uuid;

// Gettin executable of Python with output -> PathBuf || Exception as a String
pub fn get_python() -> Result<PathBuf, String> {
    // Create idiomatic python executable names in system
    let possible_names = vec!["python.exe", "python3.exe", "py.exe"];
    // Getting PATH data
    let path_var = env::var("PATH").unwrap_or_default();
    // Start looking for
    println!("Start lO_Oking for Python...");
    for search_dir in env::split_paths(&path_var) {
        println!("Looking in {}", search_dir.display());
        if let Some(python_path) = find_file_in_dir(&search_dir, &possible_names) {
            println!("Python was foun at {}", python_path.display());
            return Ok(python_path);
        }
    }
    // No found
    Err("Python not found in PATH".to_string())
}

// Create PythonInfo object on given Python executable path or Exception as String
pub fn create_python_info(python_executable_path: PathBuf) -> Result<PythonInfo, String> {
    // Run Python by it's full system path with --version arg and getting it output
    let output = Command::new(&python_executable_path) // Create a command
        .arg("--version") // add arg to command
        .output() // get command output
        .map_err(|e| format!("Exception at Python executable: {}", e))?; // if command output is std::io::Error map it to String
    // Getting version of Python executable
    // Because output structure is
    // struct Output {
    //    status: ExitStatus,
    //    stdout: Vec<u8>,
    //    stderr: Vec<u8>,
    //}
    // We getting stdout result from public getter like output.stdout and check if it is non-empty
    let version_output = if !output.stdout.is_empty() {
        // If it's true we taking info from stdout translate it to String by from_utf8_lossy
        // Note: lossy meanse that uncorrect utf-8 chars gonna be replaced with �
        String::from_utf8_lossy(&output.stdout)
    } else {
        // If stdout is empty - trynna get info from stderr
        String::from_utf8_lossy(&output.stderr)
    };
    // Clean up version output from specials
    let version_str = version_output.trim();
    Ok(PythonInfo {
        path: python_executable_path.to_path_buf(),
        version: version_str.to_string(),
    })
}

// Environment creator
pub fn make_environment() -> Result<PathBuf, String> {
    // Get Python executable from PATH
    let python_path = get_python()?;
    // Getting current working directory
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("C:\\"));
    println!("Set current working directory as {:?}", cwd);
    let environment_name = Uuid::new_v4().to_string();
    // Create venv path from env_name and cwd
    let venv_path = cwd.join(&environment_name);
    println!("Path to env is {}", venv_path.display());
    // run Python executable with flags -m venv env_name to create environment
    let exit_status = Command::new(&python_path)
        .args(&["-m", "venv", &environment_name])
        .current_dir(cwd)
        .status()
        .map_err(|e| e.to_string())?;
        // Check exit status
    if exit_status.success(){
        // Return venv_path if everytjing is ok
        Ok(venv_path)
    } else {
        // Else reutrn an error
        Err(format!("Exception at creating env: {}", exit_status))
    }
}

// Runnable of Python env
pub fn get_environment(environment_path: PathBuf) -> Result<EnvironmentInfo, String>{
    let python_bin_path = environment_path.join("Scripts").join("python.exe");
    let environment_activate_path = environment_path.join("Scripts").join("activate.bat");
    // Validate Python executable exists
    if !python_bin_path.exists() {
        return Err(format!("Smth wrong! No Python executable at {}", python_bin_path.display()));
    };
    // Validate activate script exists
    if !environment_activate_path.exists() {
        return Err(format!("Smth wrong! No environment activate sript at {}", python_bin_path.display()));
    };
    // Getting Python EnvironmentInfo
    let python_environment_info = create_python_info(python_bin_path.clone())?;
    Ok(EnvironmentInfo {
        path: environment_path,
        executable: python_bin_path,
        activate: environment_activate_path,
        version: python_environment_info.version,
    })
}

// Remover of environment
pub fn remove_environment(environment_path: PathBuf) -> Result<(), String> {
    if !environment_path.exists() {
        return Err(format!("Environment at {} is also removed", environment_path.display()));
    }
    fs::remove_dir_all(&environment_path)
        .map_err(|e| e.to_string())
        .map(|()| {
            println!("Python env successefuly removed at {}", environment_path.display());
        })
}

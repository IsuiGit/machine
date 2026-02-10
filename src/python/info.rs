use super::func::*;
use super::{
    PythonInfo,
    EnvironmentInfo
};

// Public method getting info about Python with output -> PythonInfo || Exception as String
pub fn get_python_info() -> Result<PythonInfo, String> {
    // Getting Python path from private interface get_python();
    let python_path = get_python()?;
    // Getting PythonInfo from private interface create_python_info();
    let python_info = create_python_info(&python_path)?;
    // Return PythonInfo
    Ok(python_info)
}

// Public method getting info about env with output -> PythonInfo || Exception as String
pub fn get_environment_info() -> Result<EnvironmentInfo, String> {
    let python_env = make_environment()?;
    let python_info = get_environment(&python_env)?;
    Ok(python_info)
}

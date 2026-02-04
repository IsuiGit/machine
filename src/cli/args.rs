use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Machine")]
#[command(version, about, long_about = None)]

pub struct Args{
    /// Command
    #[command(subcommand)]
    pub command: Command,
}

/// Доступные команды
#[derive(Subcommand)]
#[command(rename_all = "kebab-case")]
pub enum Command {
    /// Check Python installation
    #[command(alias = "check-py", alias="--check-py", alias="--check-python")]
    CheckPython,
    /// Check environment on device
    #[command(alias = "check-env", alias="--check-env", alias="--check-environment")]
    CheckEnvironment,
    /// Check sandbox environment capabilities
    #[command(alias = "check-sbx-c", alias="--check-sbx-c", alias="--check-sandbox-capabilities")]
    CheckSandboxCapabilities{
        /// Path to yaml file with Sandbox config
        #[arg(default_value = "")]
        path_to_yaml_file: String,
    },
    /// Execute Python script
    #[command(alias = "--run")]
    Run{
        /// Path to runnable python (.py/.pyw) script
        #[arg(default_value = "")]
        path_to_script: String,
        /// Path to sandbox config
        #[arg(default_value = "")]
        path_to_yaml_file: String,
    },
}

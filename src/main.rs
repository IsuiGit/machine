// Including all Machine modules
mod cli;
mod app;
mod sandbox;
mod python;
mod utils;
mod reciever;
mod logger;

// Use:
// - clap::Parser for parsing command line args
// - all cli funcs
// - global logger instance, struct and getter func
use clap::Parser;
use cli::*;
use logger::{init_logger, logger, LogLevel};

// Main entry point
fn main(){
    // Get command line args
    let args = Args::parse();
    // Create app instance
    let app = app::App;
    // Init logger. Match logger instance or panice!() on exception
    let init = init_logger(LogLevel::Info, "app.log".to_string());
    match init {
        Ok(()) => {},
        Err(e) => panic!("{}", e),
    }
    // Send logger initialization message
    logger().info("Logger successefuly initialized".to_string());
    // Execute command line comma
    let status = app.exec(args.command);
    // Match comma result
    match status {
        Ok(()) => logger().info("Ended with code 0".to_string()),
        Err(e) => logger().error(format!("{}", e)),
    }
}

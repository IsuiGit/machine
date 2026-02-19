mod cli;
mod app;
mod sandbox;
mod python;
mod utils;
mod reciever;
mod logger;

use clap::Parser;
use cli::*;
use logger::{init_logger, logger, LogLevel};

fn main(){
    let args = Args::parse();
    let app = app::App;
    let init = init_logger(LogLevel::Info, "app.log".to_string());
    match init {
        Ok(()) => { },
        Err(e) => panic!("{}", e),
    }
    logger().info("Logger successefuly initialized".to_string());
    let status = app.exec(args.command);
    match status {
        Ok(()) => logger().info("Ended with code 0".to_string()),
        Err(e) => logger().error(format!("{}", e)),
    }
}

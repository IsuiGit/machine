mod cli;
mod app;
mod sandbox;
mod python;
mod utils;
mod reciever;

use clap::Parser;
use cli::*;

fn main(){
    let args = Args::parse();
    let app = app::App;
    let status = app.exec(args.command);
    match status {
        Ok(()) => println!("Ended with code 0"),
        Err(e) => eprintln!("{}", e),
    }
}

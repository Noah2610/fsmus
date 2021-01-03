extern crate clap;
extern crate directories;
extern crate ron;
#[macro_use]
extern crate serde;

mod args;
mod config;

use args::{ArgCmd, Args};
use clap::Clap;
use config::Config;

fn main() {
    let args = Args::parse();
    dbg!(&args);

    match run(args) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn run(args: Args) -> Result<(), String> {
    match &args.cmd {
        ArgCmd::Start => start_server(args)?,
    }

    Ok(())
}

fn start_server(args: Args) -> Result<(), String> {
    use std::net::TcpListener;

    let config = Config::load()?;

    let tcp_listener =
        TcpListener::bind((config.host, config.port)).map_err(|e| {
            format!(
                "Couldn't bind to address \"{}:{}\"\n{}",
                config.host, config.port, e
            )
        })?;

    Ok(())
}

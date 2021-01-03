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
            eprintln!("{:?}", e);
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
    let config = Config::load()?;
    dbg!(&config);
    dbg!("{}", config.music_dir.is_dir());

    Ok(())
}

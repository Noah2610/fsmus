extern crate chrono;
extern crate clap;
extern crate directories;
extern crate rand;
extern crate rodio;
extern crate ron;
#[macro_use]
extern crate serde;

mod app;
mod args;
mod config;
mod remote;

use args::{ArgCmd, Args};
use clap::Clap;

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
    use remote::RequestMessage;

    match &args.cmd {
        ArgCmd::Start => app::start(args),
        ArgCmd::Play => remote::request(RequestMessage::Play),
        ArgCmd::Pause => remote::request(RequestMessage::Pause),
        ArgCmd::Next => remote::request(RequestMessage::Next),
    }
}

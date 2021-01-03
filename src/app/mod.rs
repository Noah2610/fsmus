mod music_player;

use crate::args::Args;
use crate::config::Config;
use music_player::MusicPlayer;

pub fn start(args: Args) -> Result<(), String> {
    let config = Config::load()?;

    let mut player = MusicPlayer::new(&config);

    // TODO
    player.play_next();
    loop {}

    start_server(&config)?;

    Ok(())
}

fn start_server(config: &Config) -> Result<(), String> {
    use std::net::TcpListener;

    let tcp_listener =
        TcpListener::bind((config.host, config.port)).map_err(|e| {
            format!(
                "Couldn't bind to address \"{}:{}\"\n{}",
                config.host, config.port, e
            )
        })?;

    Ok(())
}

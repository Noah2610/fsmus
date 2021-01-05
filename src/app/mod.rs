mod music_player;

use crate::args::Args;
use crate::config::Config;
use music_player::MusicPlayer;

pub fn start(args: Args) -> Result<(), String> {
    let config = Config::load()?;

    let mut player = MusicPlayer::new(&config);

    // TODO
    let print_err = |msg: &String| eprintln!("{}", msg);
    if let Err(e) = player
        .play_next()
        .map(|song| println!("Playing {:?}", song))
        .as_ref()
    {
        print_err(e);
    }

    start_server(&config, &mut player)?;

    Ok(())
}

fn start_server(
    config: &Config,
    music_player: &mut MusicPlayer,
) -> Result<(), String> {
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};

    let addr = (config.host, config.port);
    println!("Listening on {}:{}", addr.0, addr.1);

    let tcp_listener = TcpListener::bind(addr).map_err(|e| {
        format!(
            "Couldn't bind to address \"{}:{}\"\n{}",
            config.host, config.port, e
        )
    })?;

    for mut stream in tcp_listener.incoming() {
        match stream.as_mut() {
            Ok(stream) => {
                let mut msg = Vec::new();
                let data = {
                    let mut raw = [0; 512];
                    let _ = stream.read(&mut raw);
                    let s = String::from_utf8(raw.to_vec());
                    match s {
                        Ok(s) => s,
                        Err(e) => {
                            msg.push(format!(
                                "Request data reading error: {}",
                                e,
                            ));
                            String::new()
                        }
                    }
                };
                let now = chrono::Local::now();
                println!(
                    "--- INCOMING DATA {}\n{}\n{}",
                    now.format("%H:%M:%S"),
                    &data,
                    msg.join("\n"),
                );

                let _ = stream.write(data.as_bytes());

                stream.flush().unwrap();
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }

    Ok(())
}

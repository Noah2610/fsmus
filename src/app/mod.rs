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
    use crate::remote::RequestMessage;
    use std::io::{Read, Write};
    use std::net::TcpListener;

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
                let request: Result<RequestMessage, String> = {
                    let mut s = String::new();
                    let _ = stream.read_to_string(&mut s);
                    let deser = ron::de::from_str(&s).map_err(|e| {
                        format!("request data deserializing error: {}", e)
                    });
                    msg.push(format!("command: {}", s));
                    deser
                };

                match request {
                    Ok(request) => {
                        let res = match request {
                            RequestMessage::Play => music_player.play(),
                            RequestMessage::Pause => music_player.pause(),
                            RequestMessage::Next => {
                                music_player.play_next().map(|song| {
                                    msg.push(format!("playing: {:?}", song))
                                })
                            }
                        };
                        match res {
                            Ok(_) => (),
                            Err(e) => msg.push(e),
                        }
                    }
                    Err(e) => {
                        msg.push(e);
                    }
                }

                let now = chrono::Local::now();
                eprintln!(
                    "--- INCOMING DATA {}\n{}",
                    now.format("%H:%M:%S"),
                    msg.join("\n"),
                );

                // let _ = stream.write(data.as_bytes());
                // stream.flush().unwrap();
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }

    Ok(())
}

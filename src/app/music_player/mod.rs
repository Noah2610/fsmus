mod playlists;

use crate::config::Config;
use playlists::{Playlist, Playlists};
use rodio::{OutputStream, OutputStreamHandle};
use std::path::{Path, PathBuf};

pub struct MusicPlayer {
    stream:    (OutputStream, OutputStreamHandle),
    music_dir: PathBuf,
    playlists: Playlists,
}

impl MusicPlayer {
    pub fn new(config: &Config) -> Self {
        let stream = rodio::OutputStream::try_default().unwrap();
        let music_dir = config.music_dir.clone();
        let playlists = Playlists::from(&music_dir);

        dbg!(&playlists);

        Self {
            stream,
            music_dir,
            playlists,
        }
    }
}

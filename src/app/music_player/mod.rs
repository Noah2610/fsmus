mod playlists;

use crate::config::{Config, PlaybackBehavior};
use playlists::{Playlist, Playlists};
use rodio::{OutputStream, OutputStreamHandle};
use std::path::{Path, PathBuf};

pub struct MusicPlayer {
    stream:    (OutputStream, OutputStreamHandle),
    music_dir: PathBuf,
    playlists: Playlists,
    state:     PlaybackState,
    behavior:  PlaybackBehavior,
}

impl MusicPlayer {
    pub fn new(config: &Config) -> Self {
        let stream = rodio::OutputStream::try_default().unwrap();
        let music_dir = config.music_dir.clone();
        let playlists = Playlists::from(&music_dir);

        Self {
            stream,
            music_dir,
            playlists,
            state: PlaybackState::Stopped,
            behavior: config.playback_behavior.clone(),
        }
    }
}

#[derive(Debug)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

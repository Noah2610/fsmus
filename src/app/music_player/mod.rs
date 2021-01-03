mod playlists;

use crate::config::{Config, PlaybackBehavior};
use playlists::{Playlist, PlaylistName, Playlists};
use rodio::{OutputStream, OutputStreamHandle, Source};
use std::path::PathBuf;

pub struct MusicPlayer {
    stream:            (OutputStream, OutputStreamHandle),
    music_dir:         PathBuf,
    playlists:         Playlists,
    selected_playlist: Option<PlaylistName>,
    state:             PlaybackState,
    behavior:          PlaybackBehavior,
}

impl MusicPlayer {
    /// Create a new `MusicPlayer` from the given `Config`.
    /// Sets up audio stream and loads playlists from directories.
    pub fn new(config: &Config) -> Self {
        let stream = rodio::OutputStream::try_default().unwrap();
        let music_dir = config.music_dir.clone();
        let playlists = Playlists::from(&music_dir);

        Self {
            stream,
            music_dir,
            playlists,
            selected_playlist: config.default_playlist.clone(),
            state: PlaybackState::Stopped,
            behavior: config.playback_behavior.clone(),
        }
    }

    /// Starts playing the next song, depending on the selected playlist
    /// and the current playback behavior.
    /// Does nothing and prints a warning if no playlist is selected.
    pub fn play_next(&mut self) {
        use rand::Rng;

        if let Some(playlist) = self.get_selected_playlist() {
            match self.behavior {
                PlaybackBehavior::Random => {
                    let mut rng = rand::thread_rng();
                    let songs = playlist.get_songs();
                    let idx = rng.gen_range(0 .. songs.len());
                    self.play_audio(&songs[idx]);
                }
                PlaybackBehavior::Sequential => unimplemented!(
                    "Sequential playback behavior is unimplemented"
                ),
            }
        } else {
            eprintln!("Can't play when no playlist is selected.");
        }
    }

    fn get_selected_playlist(&self) -> Option<&Playlist> {
        self.selected_playlist
            .as_ref()
            .and_then(|name| self.playlists.get(name))
    }

    fn play_audio(&self, path: &PathBuf) -> Result<(), String> {
        use std::fs::File;
        use std::io::BufReader;

        let file = File::open(path).map_err(|e| {
            format!("Couldn't open file \"{:?}\"\n{}", &path, e)
        })?;
        let source =
            rodio::Decoder::new(BufReader::new(file)).map_err(|e| {
                format!("Couldn't play audio file \"{:?}\"\n{}", &path, e)
            })?;
        self.stream
            .1
            .play_raw(source.convert_samples())
            .map_err(|e| {
                format!("Error playing audio file \"{:?}\"\n{}", path, e)
            })?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

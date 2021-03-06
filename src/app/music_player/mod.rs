mod playlists;

use crate::config::{Config, PlaybackBehavior};
use playlists::{Playlist, PlaylistName, Playlists};
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use std::path::PathBuf;

pub struct MusicPlayer {
    stream:    (OutputStream, OutputStreamHandle),
    music_dir: PathBuf,
    playlists: Playlists,

    selected_playlist: Option<PlaylistName>,
    state:             PlaybackState,
    behavior:          PlaybackBehavior,

    playing_audio: Option<Sink>,
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
            playing_audio: None,
        }
    }

    /// Starts playing the next song, depending on the selected playlist
    /// and the current playback behavior.
    /// Does nothing and prints a warning if no playlist is selected.
    pub fn play_next(&mut self) -> Result<PathBuf, String> {
        use rand::Rng;

        if let Some(playlist) = self.get_selected_playlist() {
            match self.behavior {
                PlaybackBehavior::Random => {
                    let mut rng = rand::thread_rng();
                    let mut songs = playlist.get_songs();
                    let idx = rng.gen_range(0 .. songs.len());
                    let song = songs.remove(idx);
                    self.play_audio(&song)?;
                    Ok(song)
                }
                PlaybackBehavior::Sequential => unimplemented!(
                    "Sequential playback behavior is unimplemented"
                ),
            }
        } else {
            Err(String::from("Can't play when no playlist is selected."))
        }
    }

    /// Resumes the paused audio playback.
    pub fn play(&mut self) -> Result<(), String> {
        self.get_playing_audio()?.play();
        Ok(())
    }

    /// Pauses the playing audio playback.
    pub fn pause(&mut self) -> Result<(), String> {
        self.get_playing_audio()?.pause();
        Ok(())
    }

    pub fn seek(&mut self, time_ms: u64) -> Result<(), String> {
        Err(String::from("Seeking not yet implemented"))
    }

    fn get_playing_audio(&self) -> Result<&Sink, String> {
        self.playing_audio
            .as_ref()
            .ok_or(String::from("No playing audio sink"))
    }

    fn get_selected_playlist(&self) -> Option<&Playlist> {
        self.selected_playlist
            .as_ref()
            .and_then(|name| self.playlists.get(name))
    }

    fn play_audio(&mut self, path: &PathBuf) -> Result<(), String> {
        use rodio::Decoder;
        use std::fs::File;
        use std::io::BufReader;

        let file = File::open(path)
            .map_err(|e| format!("Couldn't open file \"{:?}\"\n{}", path, e))?;
        // let source =
        //     rodio::Decoder::new(BufReader::new(file)).map_err(|e| {
        //         format!("Couldn't play audio file \"{:?}\"\n{}", &path, e)
        //     })?;
        // let source = BufReader::new(file).buffered();
        let source = Decoder::new(BufReader::new(file))
            .map_err(|e| format!("decode error: {}", e))?;
        dbg!(source.channels());
        dbg!(source.current_frame_len());
        dbg!(source.sample_rate());
        dbg!(source.total_duration());
        let source = source.buffered();
        // let source =
        //     source.skip_duration(std::time::Duration::from_millis(1000));
        // .convert_samples()
        // .speed(1.2)
        // .buffered()
        // .low_pass(100)
        // .reverb(std::time::Duration::from_millis(500), 5.0)
        // .fade_in(std::time::Duration::from_millis(1000))
        // .amplify(50.0);
        let sink = Sink::try_new(&self.stream.1)
            .map_err(|e| format!("sink error: {}", e))?;
        sink.append(source);
        // let sink = self.stream.1.play_once(source).map_err(|e| {
        //     format!("Error playing audio file \"{:?}\"\n{}", path, e)
        // })?;
        self.playing_audio = Some(sink);

        Ok(())
    }
}

#[derive(Debug)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

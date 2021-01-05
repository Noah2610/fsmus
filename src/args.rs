use clap::Clap;

/// A headless music player, which can be controlled with the `fsmus` CLI.
/// Playlists are the directories that contain audio files
/// for the configured music directory.
#[derive(Clap, Debug)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
pub struct Args {
    #[clap(subcommand)]
    pub cmd: ArgCmd,
}

#[derive(Clap, Debug)]
pub enum ArgCmd {
    /// Start the `fsmus` server.
    Start,
    /// Resumes playback of current track.
    Play,
    /// Pauses playback of current track.
    Pause,
    /// Plays the next song from the selected playlist,
    /// depending on the playback behavior.
    Next,
}

impl Default for ArgCmd {
    fn default() -> Self {
        Self::Start
    }
}

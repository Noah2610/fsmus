use std::collections::HashMap;
use std::path::PathBuf;

const SUPPORTED_FORMATS: [&'static str; 4] = ["mp3", "wav", "flac", "ogg"];

pub type PlaylistName = String;

#[derive(Debug)]
pub struct Playlists(HashMap<PlaylistName, Playlist>);

impl Playlists {
    pub fn get(&self, name: &PlaylistName) -> Option<&Playlist> {
        self.0.get(name)
    }
}

#[derive(Debug)]
pub struct Playlist {
    path: PathBuf,
}

impl Playlist {
    /// Finds and returns a list of all audio files
    /// that belong to this playlist.
    /// Recursively searches this playlist's directory for audio files.
    pub fn get_songs(&self) -> Vec<PathBuf> {
        fn find_songs(mut songs: Vec<PathBuf>, path: &PathBuf) -> Vec<PathBuf> {
            if let Ok(entries) = path.read_dir() {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let entry_path = entry.path();
                        if is_valid_audio_file(&entry_path) {
                            songs.push(entry_path);
                        } else if entry_path.is_dir() {
                            songs = find_songs(songs, &entry_path);
                        }
                    }
                }
                songs
            } else {
                songs
            }
        }

        find_songs(Vec::new(), &self.path)
    }
}

impl From<&PathBuf> for Playlists {
    fn from(root_path: &PathBuf) -> Self {
        let root_path_s = root_path.to_str().unwrap();

        fn find_playlists(
            mut playlists: HashMap<PlaylistName, Playlist>,
            path: &PathBuf,
            root_path_s: &str,
        ) -> HashMap<PlaylistName, Playlist> {
            if let Ok(entries) = path.read_dir() {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let entry_path = entry.path();
                        if is_valid_audio_file(&entry_path) {
                            if let Some(path_s) = path.to_str() {
                                let mut playlist_key =
                                    path_s.to_string().replace(root_path_s, "");
                                while !playlist_key.is_empty() {
                                    if !playlists.contains_key(&playlist_key) {
                                        playlists.insert(
                                            playlist_key.clone(),
                                            Playlist { path: path.clone() },
                                        );
                                    }
                                    if let Some(slash_idx) =
                                        playlist_key.rfind("/")
                                    {
                                        let _ =
                                            playlist_key.split_off(slash_idx);
                                    } else {
                                        playlist_key.clear();
                                    }
                                }
                            }
                        } else if entry_path.is_dir() {
                            playlists = find_playlists(
                                playlists,
                                &entry_path,
                                root_path_s,
                            );
                        }
                    }
                }
                playlists
            } else {
                playlists
            }
        }

        Self(find_playlists(HashMap::new(), root_path, &root_path_s))
    }
}

fn is_valid_audio_file(file: &PathBuf) -> bool {
    file.is_file()
        && if let Some(extension) = file
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase())
        {
            SUPPORTED_FORMATS.iter().any(|format| format == &extension)
        } else {
            false
        }
}

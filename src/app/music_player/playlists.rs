use std::collections::HashMap;
use std::path::PathBuf;

const SUPPORTED_FORMATS: [&'static str; 4] = ["mp3", "wav", "flac", "ogg"];

#[derive(Debug)]
pub struct Playlists(HashMap<String, Playlist>);

#[derive(Debug)]
pub struct Playlist {
    path:  PathBuf,
    songs: Vec<PathBuf>,
}

impl From<&PathBuf> for Playlists {
    fn from(root_path: &PathBuf) -> Self {
        let root_path_s = root_path.to_str().unwrap();

        fn find_playlists(
            mut playlists: HashMap<String, Playlist>,
            path: &PathBuf,
            root_path_s: &str,
        ) -> HashMap<String, Playlist> {
            if let Ok(entries) = path.read_dir() {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let entry_path = entry.path();
                        if is_valid_audio_file(&entry_path) {
                            if let Some(path_s) = path.to_str() {
                                let playlist_key =
                                    path_s.to_string().replace(root_path_s, "");
                                playlists
                                    .entry(playlist_key)
                                    .or_insert_with(|| Playlist {
                                        path:  path.clone(),
                                        songs: Vec::new(),
                                    })
                                    .songs
                                    .push(entry_path);
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

use std::fs::File;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub music_dir: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let file = Self::get_config_file()?;
        let reader = File::open(file).map_err(|e| e.to_string())?;
        let mut config: Self =
            ron::de::from_reader(reader).map_err(|e| e.to_string())?;

        let base_dirs = directories::BaseDirs::new()
            .ok_or_else(|| String::from("No base directories"))?;
        let home_dir = base_dirs.home_dir().to_str().ok_or_else(|| {
            String::from("Can't convert HOME directory path to string")
        })?;
        let music_dir_s = config.music_dir.to_str().ok_or_else(|| {
            String::from("Can't convert music_dir path to string")
        })?;
        config.music_dir = PathBuf::from(music_dir_s.replace("~", home_dir));

        Ok(config)
    }

    fn get_config_file() -> Result<PathBuf, String> {
        let file = PathBuf::from("./config.ron");
        if file.is_file() {
            Ok(file)
        } else {
            let project_dirs =
                directories::ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
                    .ok_or_else(|| {
                        String::from("No project config directory")
                    })?;
            let config_dir = project_dirs.config_dir();
            let config_file = config_dir.join("config.ron");
            if config_file.is_file() {
                Ok(config_file)
            } else {
                Err(String::from(
                    "No config.ron file in config No config.ron file in \
                     config directory",
                ))
            }
        }
    }
}

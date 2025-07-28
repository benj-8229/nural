use crate::models::config_serialize::ConfigObj;
use std::{
    fs, io::Result, path::PathBuf
};
use toml;
use shellexpand::tilde;

const DEFAULT_CONFIG: &str = "\
# nural.conf - default configuration file for Nural

[general]
note_extension = \"md\"				# File extension to use for notes
editor = \"nvim\"						# Editor to use when opening notes (vim, nvim, nano, etc)
reader = \"batcat\"						# Reader to use when reading notes (cat, batcat, etc)
update_gitignore = true                 # Automatically add .nural to the .gitignore if init is ran with -g

[display]
show_full_paths = false				# Show full paths when listing notes
show_timestamps = true				# Show creation and modification timestamps
";


pub struct Config {
    pub path: PathBuf,
    pub dir: PathBuf,
}

impl Config {
    /// Construct from a specific path
    pub fn new<P: Into<PathBuf>>(path: P, dir: P) -> Self {
        Config { 
            path: path.into(),
            dir: dir.into()
        }
    }

    /// Construct from default location
    pub fn default() -> Self {
        let path = tilde("~/.config/nural/nural.conf").to_string();
        let dir = tilde("~/.config/nural").to_string();
        Config { 
            path: PathBuf::from(path),
            dir: PathBuf::from(dir),
        }
    }

    /// Check if config file exists
    pub fn config_exists(&self) -> bool {
        self.path.exists()
    }

    /// Check if config directory exists
    pub fn dir_exists(&self) -> bool {
        self.dir.exists()
    }

    pub fn create_dir(&self) -> Result<()> {
        fs::create_dir(&self.dir)
    }

    pub fn regenerate_config(&self) -> Result<()> {
        fs::write(&self.path, DEFAULT_CONFIG)
    }

    /// Read config as string
    pub fn read(&self) -> std::io::Result<String> {
        fs::read_to_string(&self.path)
    }

    pub fn get_config(&self) -> ConfigObj {
        match self.read() {
            Ok(str) => toml::from_str(&str).unwrap(),
            Err(_) => panic!("failed to read config"),
        }
    }

    // Maybe: parse config contents (if using TOML, etc.)
    // pub fn load(&self) -> Result<MyParsedConfig, ...>
}

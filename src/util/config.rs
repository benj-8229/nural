use crate::models::config_serialize::ConfigObj;
use std::{
    io::{ Result, Error },
    path::PathBuf,
    fs, 
};
use directories::ProjectDirs;
use toml;

const DEFAULT_CONFIG: &str = "\
# nural.conf - default configuration file for Nural

[general]
note_extension = \"md\"				# File extension to use for notes
editor = \"nano\"						# Editor to use when opening notes (vim, nvim, nano, etc)
reader = \"cat\"						# Reader to use when reading notes (cat, batcat, etc)
lister = \"tree\"						# Command to use to list notes (will be removed eventually)
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
    /// Construct from default location
    pub fn default() -> Self {
        let proj_dirs = ProjectDirs::from("com", "yourname", "nural").unwrap();
        let path = proj_dirs.config_dir().join("config.toml");
        let dir = proj_dirs.config_dir();
        Config { 
            path: PathBuf::from(path),
            dir: PathBuf::from(dir),
        }
    }

    pub fn default_path() -> PathBuf {
        let proj_dirs = ProjectDirs::from("com", "yourname", "nural").unwrap();
        proj_dirs.config_dir().join("config.toml")
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

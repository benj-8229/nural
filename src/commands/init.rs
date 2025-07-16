use super::ICommand;
use crate::cli::{CliEntry, Commands};
use crate::commands::utils;
use crate::config_serialize::ConfigObj;
use std::env::current_dir;
use std::path::PathBuf;
use std::io::{ErrorKind, Error};

pub struct InitCommand {
}

impl ICommand for InitCommand {
    fn execute(c_obj: ConfigObj, cli_obj: CliEntry) -> Result<(), Error> {
        if let Commands::Init { git, directory } = cli_obj.subcommand {
            let mut directory = match directory {
                Some(dir) => PathBuf::from(dir),
                None => current_dir()?,
            };
            
            // if git flag is set then update directory to backtrack to nearest project,
            // defaulting to global directory if none is found
            if git {
                match InitCommand::backtrack_to_git_dir(&directory) {
                    Some(e) => { directory = e; },
                    None => { return Err(Error::new(ErrorKind::NotFound, format!("could not find .git directory from {}", directory.to_string_lossy()))); }, // git flag + no .git = error
                }
            }

            match utils::dir_in_context(&directory, false) {
                true => { return Err(Error::new(ErrorKind::DirectoryNotEmpty, format!("{} already has context", directory.to_string_lossy()))); },
                false => {
                    let mut tmp = directory.clone();
                    tmp.push(".nural");
                    std::fs::create_dir(tmp)?;
                }
            }

            if c_obj.general.update_gitignore {
                // implement auto updating gitignore
            }
        }
        Ok(())
    } 
}

impl InitCommand {
    // recursively backtrack to try to find a directory with .git
    fn backtrack_to_git_dir(cur_dir: &PathBuf) -> Option<PathBuf> {
        if cur_dir.iter().count() == 0 as usize {
            return None;
        }

        let mut tmp_path = cur_dir.clone();
        tmp_path.push(".git");

        if tmp_path.try_exists().expect("failed to check directory existence") {
            return Some(cur_dir.into());
        }

        let mut new_path = cur_dir.clone();
        new_path.pop();
        InitCommand::backtrack_to_git_dir(&new_path)
    }
}

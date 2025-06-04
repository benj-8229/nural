use super::ICommand;
use crate::cli::{CliEntry, Commands};
use crate::config_serialize::ConfigObj;
use std::env::current_dir;
use std::path::PathBuf;

pub struct InitCommand {
}

impl ICommand for InitCommand {
    fn execute(c_obj: ConfigObj, cli_obj: CliEntry) -> Result<(), std::io::Error> {
        if let Commands::Init { git, directory } = cli_obj.subcommand {
            //
            let mut directory = match directory {
                Some(dir) => PathBuf::from(dir),
                None => current_dir()?,
            };
            
            // if git flag is set then update directory to backtrack to nearest project,
            // defaulting to global directory if none is found
            if git {
                match InitCommand::backtrack_to_git_dir(directory) {
                    Some(e) => { directory = e; },
                    None => { directory = PathBuf::from(c_obj.general.global_dir); },
                }
            }

            match InitCommand::dir_has_context(&directory) {
                true => { return Err(std::io::Error::new(std::io::ErrorKind::DirectoryNotEmpty, "directory already has context")); },
                false => {
                    let mut tmp = directory.clone();
                    tmp.push(".nural");
                    std::fs::create_dir(tmp)?;
                }
            }

            if c_obj.general.update_gitignore {
                // implement auto updating gitignore, potentially too destructive for haphazard implementation
            }
        }
        Ok(())
    } 
}

impl InitCommand {
    // recursively backtrack to try to find a directory with .git
    fn backtrack_to_git_dir(cur_dir: PathBuf) -> Option<PathBuf> {
        if cur_dir.iter().count() == 0 as usize {
            return None;
        }

        let mut tmp_path = cur_dir.clone();
        tmp_path.push(".git");

        if tmp_path.try_exists().expect("failed to check directory existence") {
            return Some(cur_dir);
        }

        let mut new_path = cur_dir.clone();
        new_path.pop();
        InitCommand::backtrack_to_git_dir(new_path)
    }

    // check if note context already exists in a directory
    fn dir_has_context(dir: &PathBuf) -> bool {
        let mut tmp = dir.clone();
        tmp.push(".nural");
        tmp.exists()
    }
}

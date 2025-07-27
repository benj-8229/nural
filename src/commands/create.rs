use super::ICommand;
use crate::util::cli::{CliEntry, Commands};
use crate::commands::utils;
use crate::models::config_serialize::ConfigObj;
use std::env::current_dir;
use std::path::PathBuf;
use std::io::{ErrorKind, Error};

pub struct CreateCommand {
}

impl ICommand for CreateCommand {
    fn execute(conf_obj: ConfigObj, cli_obj: CliEntry) -> Result<(), Error> {
        if let Commands::Create { name: note_name, } = cli_obj.subcommand {
            let global_path = PathBuf::from(utils::expand_dir(&conf_obj.general.global_dir));

            // return global_path if global=true or if the current directory has no context
            let context_dir = match cli_obj.global {
                true => global_path,
                false => utils::get_dir_context(&current_dir()?).unwrap_or(global_path),
            };

            let mut note_path = context_dir.clone(); 
            note_path.push(format!("{}.{}", note_name, conf_obj.general.note_extension));

            if note_path.exists() {
                return Err(Error::new(ErrorKind::AlreadyExists, format!("note with name {} already exists", note_name)));
            }

            std::fs::write(note_path, "")?

            // todo: implement tagging with metadata file in context root
        }

        Ok(())
    }
}

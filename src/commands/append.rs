use super::ICommand;
use crate::cli::{CliEntry, Commands};
use crate::commands::utils;
use crate::config_serialize::ConfigObj;
use crate::note_query;
use std::fs;
use std::env::current_dir;
use std::path::PathBuf;
use std::io::{Error};

pub struct AppendCommand {
}

impl ICommand for AppendCommand {
    fn execute(conf_obj: ConfigObj, cli_obj: CliEntry) -> Result<(), Error> {
        if let Commands::Append { name: note_name, tag: _tags, text: _append_text } = cli_obj.subcommand {
            let global_path = PathBuf::from(utils::expand_dir(&conf_obj.general.global_dir));

            // return global_path if global=true or if the current directory has no context
            let context_dir = match cli_obj.global {
                true => global_path,
                false => utils::get_dir_context(&current_dir()?).unwrap_or(global_path),
            };
            
            let note_path = context_dir.clone(); 
            let mut file_list: Vec<PathBuf> = Vec::new(); 
            let files = fs::read_dir(note_path).unwrap();
            for file in files
            {
                file_list.push(file.unwrap().path());
            }
            
            let result = note_query::query_tui(file_list, note_name.unwrap_or(String::from("")));
            println!("{:?}", result);
        }

        Ok(())
    }
}


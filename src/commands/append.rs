use super::ICommand;
use crate::cli::{CliEntry, Commands};
use crate::commands::utils;
use crate::config_serialize::ConfigObj;
use crate::fuzzy;
use std::fs;
use std::env::current_dir;
use std::path::PathBuf;
use std::io::{ErrorKind, Error};

pub struct AppendCommand {
}

impl ICommand for AppendCommand {
    fn execute(conf_obj: ConfigObj, cli_obj: CliEntry) -> Result<(), Error> {
        if let Commands::Append { name: note_name, tag: _tags, text: append_text } = cli_obj.subcommand {
            let global_path = PathBuf::from(utils::expand_dir(&conf_obj.general.global_dir));

            // return global_path if global=true or if the current directory has no context
            let context_dir = match cli_obj.global {
                true => global_path,
                false => utils::get_dir_context(&current_dir()?).unwrap_or(global_path),
            };
            
            let note_path = context_dir.clone(); 
            let mut file_list: Vec<String> = Vec::new(); 
            let files = fs::read_dir(note_path).unwrap();
            for file in files
            {
                let file_path = file.unwrap().path().to_str().expect("bad file name").to_string();
                file_list.push(file_path);
            }
            
            let scores = fuzzy::score_options(file_list, note_name.expect("no note name provided (temp)"));
            println!("{:?}", scores);
        }

        Ok(())
    }
}


use super::ICommand;
use std::io::ErrorKind;
use crate::util::cli::{CliEntry, Commands};
use crate::commands::utils;
use crate::models::{config_serialize::ConfigObj, context};
use crate::util::note_query;

use std::fs;
use std::env::current_dir;
use std::path::PathBuf;
use std::io::{Error, Write};

pub struct AppendCommand {
}

impl ICommand for AppendCommand {
    fn execute(conf_obj: ConfigObj, cli_obj: CliEntry) -> Result<(), Error> {
        if let Commands::Append { name, text } = cli_obj.subcommand {
            let _global_path: PathBuf = PathBuf::from(utils::expand_dir(&conf_obj.general.global_dir));
            let context: Option<context::Context> = context::get_dir_context(&current_dir()?);
            let context = context.ok_or(Error::new(ErrorKind::NotFound, "context not found"))?;

            /* Going to reimplement global contexts at a later time
            // return global_path if global=true or if the current directory has no context
            let context_dir = match cli_obj.global {
                true => global_path,
                false => utils::get_dir_context(&current_dir()?).unwrap_or(global_path),
            };
            */

            let note_path = &context.dir; 
            let mut file_list: Vec<PathBuf> = Vec::new(); 
            let files = fs::read_dir(note_path).unwrap();
            for file in files
            {
                file_list.push(file.unwrap().path());
            }
            
            match note_query::query_tui(context, name.unwrap_or(String::from(""))) {
                Ok(res) => {
                    let mut file = std::fs::OpenOptions::new()
                        .write(true)
                        .append(true)
                        .open(res.path)?;
                    writeln!(file, "{}", text)?;
                }
                Err(e) => { return Err(e); }
            }
        }

        Ok(())
    }
}


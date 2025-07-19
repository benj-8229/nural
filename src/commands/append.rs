use super::ICommand;
use std::io::ErrorKind;
use crate::cli::{CliEntry, Commands};
use crate::commands::utils;
use crate::config_serialize::ConfigObj;
use crate::note_query;
use std::fs;
use std::env::current_dir;
use std::path::PathBuf;
use std::io::{Error, Write};

pub struct AppendCommand {
}

impl ICommand for AppendCommand {
    fn execute(conf_obj: ConfigObj, cli_obj: CliEntry) -> Result<(), Error> {
        if let Commands::Append { name, create, text } = cli_obj.subcommand {
            let global_path = PathBuf::from(utils::expand_dir(&conf_obj.general.global_dir));

            // return global_path if global=true or if the current directory has no context
            let context_dir = match cli_obj.global {
                true => global_path,
                false => utils::get_dir_context(&current_dir()?).unwrap_or(global_path),
            };

            match create {
                Some(create_name) => { 
                    let mut note_path = context_dir.clone(); 
                    note_path.push(format!("{}.{}", create_name, conf_obj.general.note_extension));

                    if note_path.exists() {
                        return Err(Error::new(ErrorKind::AlreadyExists, format!("note with name {} already exists", create_name)));
                    }

                    std::fs::write(note_path, &text)?;
                    return Ok(());
                }
                None => { }
            }
            
            let note_path = context_dir.clone(); 
            let mut file_list: Vec<PathBuf> = Vec::new(); 
            let files = fs::read_dir(note_path).unwrap();
            for file in files
            {
                file_list.push(file.unwrap().path());
            }
            
            match note_query::query_tui(file_list, name.unwrap_or(String::from(""))) {
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


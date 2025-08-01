use super::ICommand;
use crate::models::{config_serialize::ConfigObj, context};
use crate::util::cli::{CliEntry, Commands};
use crate::util::note_query;
use std::io::{self, ErrorKind};

use std::env::current_dir;
use std::io::Error;

pub struct DeleteCommand {}

impl ICommand for DeleteCommand {
    fn execute(_conf_obj: &ConfigObj, cli_obj: &CliEntry) -> Result<(), Error> {
        if let Some(Commands::Delete { name }) = &cli_obj.subcommand {
            let context: Option<context::Context> = context::get_dir_context(&current_dir()?);
            let context = context.ok_or(Error::new(ErrorKind::NotFound, "context not found"))?;

            let query = note_query::query_tui(context, name.to_owned().unwrap_or(String::from("")))?;

            let mut user_input: String = Default::default();
            println!(
                "Are you sure you want to delete {}? [y]es / [n]o: ",
                query.note.name
            );
            io::stdin().read_line(&mut user_input)?;

            match user_input.to_lowercase().chars().nth(0) {
                Some('y') => {
                    return Ok(std::fs::remove_file(query.note.path)?);
                }
                _ => {
                    println!("aborted delete");
                }
            }
        }

        Ok(())
    }
}

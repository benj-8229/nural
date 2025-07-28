use super::ICommand;
use std::io::{self, ErrorKind};
use crate::util::cli::{CliEntry, Commands};
use crate::models::{config_serialize::ConfigObj, context};
use crate::util::note_query;

use std::env::current_dir;
use std::io::{Error};

pub struct DeleteCommand {
}

impl ICommand for DeleteCommand {
    fn execute(_conf_obj: ConfigObj, cli_obj: CliEntry) -> Result<(), Error> {
        if let Some(Commands::Delete { name }) = cli_obj.subcommand {
            let context: Option<context::Context> = context::get_dir_context(&current_dir()?);
            let context = context.ok_or(Error::new(ErrorKind::NotFound, "context not found"))?;

            let matched_note = note_query::query_tui(context, name.unwrap_or(String::from("")))?;

            let mut user_input: String = Default::default();
            println!("Are you sure you want to delete {}? [y]es / [n]o: ", matched_note.filename);
            io::stdin().read_line(&mut user_input)?;

            match user_input.to_lowercase().chars().nth(0) {
                Some('y') => { return Ok(std::fs::remove_file(matched_note.path)?); },
                _ => { println!("aborted delete"); },
            }
        }

        Ok(())
    }
}

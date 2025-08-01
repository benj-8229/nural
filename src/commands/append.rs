use super::ICommand;
use std::io::ErrorKind;
use crate::util::cli::{CliEntry, Commands};
use crate::models::{config_serialize::ConfigObj, context};
use crate::util::note_query;

use std::env::current_dir;
use std::io::{Error, Write};

pub struct AppendCommand {
}

impl ICommand for AppendCommand {
    fn execute(_conf_obj: ConfigObj, cli_obj: CliEntry) -> Result<(), Error> {
        if let Some(Commands::Append { name, text }) = cli_obj.subcommand {
            let context: Option<context::Context> = context::get_dir_context(&current_dir()?);
            let context = context.ok_or(Error::new(ErrorKind::NotFound, "context not found"))?;

            let query = note_query::query_tui(context, name.unwrap_or(String::from("")))?;
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(query.note.path)?;
            writeln!(file, "{}", text)?;
            println!("appended to {}", query.note.name);
        }

        Ok(())
    }
}


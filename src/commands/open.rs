use super::ICommand;
use std::io::ErrorKind;
use crate::util::cli::{CliEntry, Commands};
use crate::models::{config_serialize::ConfigObj, context};
use crate::util::note_query;

use std::process::Command;
use std::env::current_dir;
use std::io::{Error};

pub struct OpenCommand {
}

impl ICommand for OpenCommand {
    fn execute(conf_obj: ConfigObj, cli_obj: CliEntry) -> Result<(), Error> {
        if let Some(Commands::Open { name }) = cli_obj.subcommand {
            let editor: String = conf_obj.general.editor;
            let context: Option<context::Context> = context::get_dir_context(&current_dir()?);
            let context = context.ok_or(Error::new(ErrorKind::NotFound, "context not found"))?;

            let matched_note = note_query::query_tui(context, name.unwrap_or(String::from("")))?;
            Command::new(editor)
                .arg(matched_note.path.display().to_string())
                .status()?;
        }

        Ok(())
    }
}

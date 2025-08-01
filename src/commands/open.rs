use super::ICommand;
use crate::models::{config_serialize::ConfigObj, context};
use crate::util::cli::{CliEntry, Commands};
use crate::util::note_query;
use std::io::ErrorKind;

use std::env::current_dir;
use std::io::Error;
use std::process::Command;

pub struct OpenCommand {}

impl ICommand for OpenCommand {
    fn execute(conf_obj: ConfigObj, cli_obj: CliEntry) -> Result<(), Error> {
        if let Some(Commands::Open { name }) = cli_obj.subcommand {
            let editor: String = conf_obj.general.editor;
            let context: Option<context::Context> = context::get_dir_context(&current_dir()?);
            let context = context.ok_or(Error::new(ErrorKind::NotFound, "context not found"))?;

            let query = note_query::query_tui(context, name.unwrap_or(String::from("")))?;
            Command::new(editor)
                .arg(query.note.path.display().to_string())
                .status()?;
        }

        Ok(())
    }
}

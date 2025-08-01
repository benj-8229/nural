use super::ICommand;
use crate::models::{config_serialize::ConfigObj, context};
use crate::util::cli::{CliEntry, Commands};
use crate::util::note_query;
use std::io::ErrorKind;

use std::env::current_dir;
use std::io::Error;
use std::process::{Command, Stdio};

pub struct ReadCommand {}

impl ICommand for ReadCommand {
    fn execute(conf_obj: ConfigObj, cli_obj: CliEntry) -> Result<(), Error> {
        if let Some(Commands::Read { name }) = cli_obj.subcommand {
            let reader: String = conf_obj.general.reader;
            let context: Option<context::Context> = context::get_dir_context(&current_dir()?);
            let context = context.ok_or(Error::new(ErrorKind::NotFound, "context not found"))?;

            let query_match = note_query::query_tui(context, name.unwrap_or(String::from("")))?;

            Command::new(reader)
                .arg(query_match.note.path.display().to_string())
                .stdin(Stdio::inherit()) // optional: pass input through too
                .stdout(Stdio::inherit()) // <- stream output directly
                .stderr(Stdio::inherit()) // <- stream errors too
                .spawn()?
                .wait()?; // wait for process to finish
        }

        Ok(())
    }
}

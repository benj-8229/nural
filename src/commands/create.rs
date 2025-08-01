use super::ICommand;
use crate::models::config_serialize::ConfigObj;
use crate::models::context;
use crate::util::cli::{CliEntry, Commands};
use std::env::current_dir;
use std::io::{Error, ErrorKind};

pub struct CreateCommand {}

impl ICommand for CreateCommand {
    fn execute(conf_obj: ConfigObj, cli_obj: CliEntry) -> Result<(), Error> {
        if let Some(Commands::Create { name: note_name }) = cli_obj.subcommand {
            let context: Option<context::Context> = context::get_dir_context(&current_dir()?);
            let context = context.ok_or(Error::new(ErrorKind::NotFound, "context not found"))?;

            let mut note_path = context.dir.clone();
            note_path.push(format!("{}.{}", note_name, conf_obj.general.note_extension));

            if note_path.exists() {
                return Err(Error::new(
                    ErrorKind::AlreadyExists,
                    format!("note with name {} already exists", note_name),
                ));
            }

            std::fs::write(note_path, "")?;
            println!("created note {}", note_name);
        }

        Ok(())
    }
}

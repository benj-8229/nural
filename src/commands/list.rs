use super::ICommand;
use std::io::ErrorKind;
use crate::util::cli::{CliEntry, Commands};
use crate::models::{config_serialize::ConfigObj, context};

use std::process::{Command, Stdio};
use std::env::current_dir;
use std::io::{Error};

pub struct ListCommand {
}

impl ICommand for ListCommand {
    fn execute(conf_obj: ConfigObj, cli_obj: CliEntry) -> Result<(), Error> {
        if let Some(Commands::List { }) = cli_obj.subcommand {
            let lister: String = conf_obj.general.lister;
            let context: Option<context::Context> = context::get_dir_context(&current_dir()?);
            let context = context.ok_or(Error::new(ErrorKind::NotFound, "context not found"))?;

            Command::new(lister)
                .arg(context.dir.display().to_string())
                .stdout(Stdio::inherit()) 
                .stderr(Stdio::inherit())
                .spawn()?
                .wait()?;  // wait for process to finish
        }

        Ok(())
    }
}



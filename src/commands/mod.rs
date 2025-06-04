use crate::{cli::CliEntry, config_serialize};
pub mod init;

pub trait ICommand {
    fn execute(config_obj: config_serialize::ConfigObj, cli_obj: CliEntry) -> std::io::Result<()>;
}

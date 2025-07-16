use crate::{cli::CliEntry, config_serialize::ConfigObj};

pub mod utils;
pub mod init;
pub mod create;
pub mod append;

pub trait ICommand {
    fn execute(config_obj: ConfigObj, cli_obj: CliEntry) -> std::io::Result<()>;
}

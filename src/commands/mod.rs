use crate::util::cli::CliEntry;
use crate::models::config_serialize::ConfigObj;

pub mod init;
pub mod create;
pub mod append;
pub mod open;
pub mod delete;
pub mod read;

pub trait ICommand {
    fn execute(config_obj: ConfigObj, cli_obj: CliEntry) -> std::io::Result<()>;
}

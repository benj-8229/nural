use crate::models::config_serialize::ConfigObj;
use crate::util::cli::CliEntry;

pub mod append;
pub mod create;
pub mod delete;
pub mod init;
pub mod list;
pub mod open;
pub mod read;

pub trait ICommand {
    fn execute(config_obj: &ConfigObj, cli_obj: &CliEntry) -> std::io::Result<()>;
}
